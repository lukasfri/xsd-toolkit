#[test]
fn specification_test_1() {
    // <xs:complexType name="length1">
    //     <xs:simpleContent>
    //         <xs:extension base="xs:nonNegativeInteger">
    //             <xs:attribute name="unit" type="xs:NMTOKEN"/>
    //         </xs:extension>
    //     </xs:simpleContent>
    // </xs:complexType>

    // <xs:element name="width" type="length1"/>

    let length1 = TopLevelComplexType {
        name: LocalName::new(value),
        content: ComplexTypeModel::SimpleContent(SimpleContent {
            content: ExtensionType {
                particle: todo!(),
                id: todo!(),
                base: todo!(),
                annotation: todo!(),
                content: todo!(),
            },
            id: todo!(),
        }),
        id: todo!(),
        mixed: todo!(),
        abstract_: todo!(),
        final_: todo!(),
        block: todo!(),
        default_attributes_apply: todo!(),
        annotation: todo!(),
    };
    // <width unit="cm">25</width>
}

#[test]
fn specification_test_2() {
    let mut compiled_namespace =
        CompiledNamespace::new(XmlNamespace::new_dangerous("http://localhost"));

    // <xs:complexType name="length2">
    //     <xs:complexContent>
    //         <xs:restriction base="xs:anyType">
    //             <xs:sequence>
    //                 <xs:element name="size" type="xs:nonNegativeInteger"/>
    //                 <xs:element name="unit" type="xs:NMTOKEN"/>
    //             </xs:sequence>
    //         </xs:restriction>
    //     </xs:complexContent>
    // </xs:complexType>
    let length2 = TopLevelComplexType {
        name: LocalName::new_dangerous("length2"),
        id: todo!(),
        mixed: todo!(),
        abstract_: todo!(),
        final_: todo!(),
        block: todo!(),
        default_attributes_apply: todo!(),
        annotation: todo!(),
        content: ComplexTypeModel::ComplexContent(()),
    };

    let length2 = compiled_namespace.add_top_level_complex_type(&length2);

    // <xs:element name="depth" type="length2"/>
    let depth = TopLevelElement {
        name: LocalName::new_dangerous("depth"),
        type_: Type(QName(ExpandedName::new(
            LocalName::new_dangerous("length2"),
            None,
        ))),
        id: todo!(),
        substitution_group: todo!(),
        default: todo!(),
        fixed: todo!(),
        nillable: todo!(),
        abstract_: todo!(),
        final_: todo!(),
        block: todo!(),
        form: todo!(),
        target_namespace: todo!(),
        annotation: todo!(),
        type_choice: todo!(),
        alternatives: todo!(),
        identity_constraints: todo!(),
    };

    let depth = compiled_namespace.add_top_level_element(&depth);
    compiled_namespace.transform(ExpandBasedFragments::new());

    let mut xmlns_context = XmlnsContext::new();
    xmlns_context.add_namespace(compiled_namespace);

    // let top_level_element = xmlns_context
    //     .namespaces
    //     .get(element.namespace().unwrap())
    //     .unwrap()
    //     .top_level_types
    //     .get(element.local_name());

    let fragments = xmlns_context
        .namespaces
        .get(element.namespace().unwrap())
        .unwrap();

    let fragments = &fragments.complex_type.fragments;

    println!("{fragments:#?}");

    // <depth>
    //     <size>25</size><unit>cm</unit>
    // </depth>
}

#[test]
fn specification_test_3() {
    // <xs:complexType name="length3">
    //     <xs:sequence>
    //         <xs:element name="size" type="xs:nonNegativeInteger"/>
    //         <xs:element name="unit" type="xs:NMTOKEN"/>
    //     </xs:sequence>
    // </xs:complexType>

    // <xs:element name="depth" type="length3"/>

    // <depth>
    //     <size>25</size><unit>cm</unit>
    // </depth>
}
