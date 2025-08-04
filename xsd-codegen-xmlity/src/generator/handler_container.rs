use std::sync::{Arc, Weak};

use crate::{
    complex::{
        attributes::{
            AnyAttributesHandler, AttributeDeclarationHandler, AttributeDeclarationsHandler,
            LocalAttributeHandler, TopLevelAttributeHandler,
        },
        complex_type::{
            ComplexContentHandler, ComplexTypeModelHandler, ComplexTypeRootHandler,
            RestrictionHandler, SimpleContentFragmentHandler, SimpleExtensionFragmentHandler,
        },
        elements::{
            DeclaredElementHandler, ElementTypeContentIdHandler, LocalElementHandler,
            ReferenceElementHandler, TopLevelElementHandler,
        },
        groups::{
            AllFragmentHandler, AnyFragmentHandler, ChoiceFragmentHandler, GroupRefFragmentHandler,
            NamedGroupTypeContentIdHandler, NestedParticleIdHandler, SequenceFragmentHandler,
            TopLevelGroupHandler, TypeDefParticleIdHandler,
        },
    },
    naming_strategies::{IndexedNamingStrategy, WrappingNamingStrategy},
    simple::simple_type::SimpleTypeRootHandler,
};

#[derive(Debug)]
pub struct HandlerContainer {
    pub nested_particle_handler: Arc<NestedParticleIdHandler>,
    pub simple_type_root_handler: Arc<SimpleTypeRootHandler>,
    pub complex_type_root_handler: Arc<ComplexTypeRootHandler>,
    pub top_level_attribute_handler: Arc<TopLevelAttributeHandler>,
    pub top_level_element_handler: Arc<TopLevelElementHandler>,
    pub top_level_group_handler: Arc<TopLevelGroupHandler>,
}

pub fn handler_container() -> HandlerContainer {
    let group_ref_handler = Arc::new(GroupRefFragmentHandler {});

    let any_handler = Arc::new(AnyFragmentHandler);

    let simple_type_root_handler = Arc::new(SimpleTypeRootHandler {});

    let local_attribute_handler = Arc::new(LocalAttributeHandler {
        simple_type_handler: simple_type_root_handler.clone(),
        value_type_naming: WrappingNamingStrategy::new(|name| format!("{}Value", name)),
    });

    let any_attributes_handler = Arc::new(AnyAttributesHandler {
        any_attributes_ident: "AnyAttributes".to_string(),
    });

    let attribute_declaration_handler = Arc::new(AttributeDeclarationHandler {
        local_attribute_handler,
    });

    let attribute_declarations_handler = Arc::new(AttributeDeclarationsHandler {
        any_attributes_handler,
        attribute_declaration_handler,
        suggested_attribute_type_naming: IndexedNamingStrategy::new(|index| {
            format!("Attribute{index}")
        }),
        default_attribute_ident_naming: IndexedNamingStrategy::new(|index| {
            format!("attribute_{index}")
        }),
    });

    let simple_extension_handler = Arc::new(SimpleExtensionFragmentHandler {
        attribute_declarations_handler: attribute_declarations_handler.clone(),
        content_field_ident: "content".to_string(),
        attribute_suffix_naming: WrappingNamingStrategy::new(|name| {
            if name.ends_with("_") {
                format!("{name}attribute")
            } else {
                format!("{name}_attribute")
            }
        }),
    });

    let simple_content_handler = Arc::new(SimpleContentFragmentHandler {
        simple_extension_handler,
    });

    let mut all_handler = Arc::new(AllFragmentHandler {
        nested_particle_handler: Weak::new(),
        child_naming: IndexedNamingStrategy::new(|index| format!("Child{}", index)),
        mod_naming: WrappingNamingStrategy::new(|name| format!("{}_items", name)),
    });

    let mut sequence_handler = Arc::new(SequenceFragmentHandler {
        nested_particle_handler: Weak::new(),
        child_naming: IndexedNamingStrategy::new(|index| format!("Child{}", index)),
        mod_naming: WrappingNamingStrategy::new(|name| format!("{}_items", name)),
    });

    let mut choice_handler = Arc::new(ChoiceFragmentHandler {
        nested_particle_handler: Weak::new(),
        variant_naming: IndexedNamingStrategy::new(|index| format!("Variant{}", index)),
        mod_naming: WrappingNamingStrategy::new(|name| format!("{}_variants", name)),
    });

    let mut element_type_content_handler_export = None;
    let mut complex_type_root_handler_export = None;

    let nested_particle_handler = Arc::new_cyclic(|nested_particle_handler| {
        Arc::get_mut(&mut all_handler)
            .expect("all_handler is not initialized")
            .nested_particle_handler = nested_particle_handler.clone();
        Arc::get_mut(&mut sequence_handler)
            .expect("sequence_handler is not initialized")
            .nested_particle_handler = nested_particle_handler.clone();
        Arc::get_mut(&mut choice_handler)
            .expect("choice_handler is not initialized")
            .nested_particle_handler = nested_particle_handler.clone();

        let type_def_particle_handler = Arc::new(TypeDefParticleIdHandler {
            group_ref_handler: group_ref_handler.clone(),
            all_handler: all_handler.clone(),
            choice_handler: choice_handler.clone(),
            sequence_handler: sequence_handler.clone(),
        });

        let restriction_fragment_handler = Arc::new(RestrictionHandler {
            attribute_declarations_handler: attribute_declarations_handler.clone(),
            type_def_particle_handler: type_def_particle_handler.clone(),
            default_particle_ident: "Particle".to_string(),
            content_type_naming: WrappingNamingStrategy::new(|name| format!("{name}Content")),
            attribute_suffix_naming: WrappingNamingStrategy::new(|name| {
                if name.ends_with("_") {
                    format!("{name}attribute")
                } else {
                    format!("{name}_attribute")
                }
            }),
        });

        let complex_content_handler = Arc::new(ComplexContentHandler {
            restriction_fragment_handler,
        });

        let complex_type_model_handler = Arc::new(ComplexTypeModelHandler {
            simple_content_handler,
            complex_content_handler,
            attribute_declarations_handler,
            type_def_particle_handler,
            other_content_type_naming: WrappingNamingStrategy::new(|name| {
                format!("{}Content", name)
            }),
            other_content_field_ident: "content".to_string(),
        });

        let complex_type_root_handler = Arc::new(ComplexTypeRootHandler {
            complex_type_model_handler,
        });

        complex_type_root_handler_export = Some(complex_type_root_handler.clone());

        let element_type_content_handler = Arc::new(ElementTypeContentIdHandler {
            simple_type_root_handler: simple_type_root_handler.clone(),
            complex_type_root_handler,
        });

        element_type_content_handler_export = Some(element_type_content_handler.clone());

        let declared_element_handler = Arc::new(DeclaredElementHandler {
            element_type_content_handler,
        });

        let reference_element_handler = Arc::new(ReferenceElementHandler {});

        let local_element_handler = Arc::new(LocalElementHandler {
            declared_element_handler,
            reference_element_handler,
        });

        NestedParticleIdHandler {
            local_element_handler,
            group_ref_handler: group_ref_handler.clone(),
            choice_handler: choice_handler.clone(),
            sequence_handler: sequence_handler.clone(),
            any_handler: any_handler.clone(),
            mod_naming: WrappingNamingStrategy::new(|name| format!("{}_items", name)),
        }
    });

    let element_type_content_handler =
        element_type_content_handler_export.expect("Element type content handler export");
    let complex_type_root_handler =
        complex_type_root_handler_export.expect("Complex type root handler export");

    let top_level_attribute_handler = Arc::new(TopLevelAttributeHandler);

    let top_level_element_handler = Arc::new(TopLevelElementHandler {
        dynamic_substitute_group: true,
        standalone_element_type: true,
        element_type_content_handler,
        dynamic_variant_ident: "Dynamic".to_string(),
    });

    let named_group_type_content_handler = Arc::new(NamedGroupTypeContentIdHandler {
        all_handler: Arc::downgrade(&all_handler),
        sequence_handler: Arc::downgrade(&sequence_handler),
        choice_handler: Arc::downgrade(&choice_handler),
    });

    let top_level_group_handler = Arc::new(TopLevelGroupHandler {
        named_group_type_content_handler,
    });

    HandlerContainer {
        nested_particle_handler,
        simple_type_root_handler,
        complex_type_root_handler,
        top_level_attribute_handler,
        top_level_element_handler,
        top_level_group_handler,
    }
}
