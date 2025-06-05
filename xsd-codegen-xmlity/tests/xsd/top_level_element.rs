use quote::ToTokens;
use syn::parse_quote;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::schema as xs;
use xsd_codegen_xmlity::{misc::TypeReference, BoundType, Generator, TypeType};
use xsd_type_compiler::{
    complex::transformers::{
        ExpandAttributeGroups, ExpandExtensionFragments, ExpandRestrictionFragments,
        RemoveProhibitedAttributes,
    },
    transformers::TransformChange,
    CompiledNamespace, XmlnsContext,
};

#[test]
fn temp_generate_types() {
    let open_attrs = r###"
    <xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="openAttrs">
      <xs:annotation>
        <xs:documentation>
        This type is extended by almost all schema types
        to allow attributes from other namespaces to be
        added to user schemas.
      </xs:documentation>
      </xs:annotation>
      <xs:complexContent>
        <xs:restriction base="xs:anyType">
          <xs:anyAttribute namespace="##other" processContents="lax"/>
        </xs:restriction>
      </xs:complexContent>
    </xs:complexType>
    "###
    .trim();

    let annotated = r###"
    <xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="annotated">
      <xs:annotation>
        <xs:documentation>
         This type is extended by all types which allow annotation
         other than &lt;schema> itself
       </xs:documentation>
      </xs:annotation>
      <xs:complexContent>
        <xs:extension base="xs:openAttrs">
          <xs:sequence>
            <xs:element ref="xs:annotation" minOccurs="0"/>
          </xs:sequence>
          <xs:attribute name="id" type="xs:ID"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
    "###
    .trim();

    let element = r###"
    <xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="element" abstract="true">
      <xs:annotation>
        <xs:documentation>
     The element element can be used either
     at the top level to define an element-type binding globally,
     or within a content model to either reference a globally-defined
     element or type or declare an element-type binding locally.
     The ref form is not allowed at the top level.</xs:documentation>
      </xs:annotation>
      <xs:complexContent>
        <xs:extension base="xs:annotated">
          <xs:sequence>
            <xs:choice minOccurs="0">
              <xs:element name="simpleType" type="xs:localSimpleType"/>
              <xs:element name="complexType" type="xs:localComplexType"/>
            </xs:choice>
            <xs:element name="alternative" type="xs:altType"
                      minOccurs="0" maxOccurs="unbounded"/>
            <xs:group ref="xs:identityConstraint" minOccurs="0"
                      maxOccurs="unbounded"/>
          </xs:sequence>
          <xs:attributeGroup ref="xs:defRef"/>
          <xs:attribute name="type" type="xs:QName"/>

          <xs:attribute name="substitutionGroup">
           <xs:simpleType>
            <xs:list itemType="xs:QName"/>
           </xs:simpleType>
          </xs:attribute>
          <xs:attributeGroup ref="xs:occurs"/>
          <xs:attribute name="default" type="xs:string"/>
          <xs:attribute name="fixed" type="xs:string"/>
          <xs:attribute name="nillable" type="xs:boolean" use="optional"/>
          <xs:attribute name="abstract" type="xs:boolean" default="false"
                        use="optional"/>
          <xs:attribute name="final" type="xs:derivationSet"/>
          <xs:attribute name="block" type="xs:blockSet"/>
          <xs:attribute name="form" type="xs:formChoice"/>
          <xs:attribute name="targetNamespace" type="xs:anyURI"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
    "###
    .trim();

    let top_level_element = r###"
    <xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="topLevelElement">
      <xs:complexContent>
        <xs:restriction base="xs:element">
          <xs:sequence>
            <xs:element ref="xs:annotation" minOccurs="0"/>
            <xs:choice minOccurs="0">
              <xs:element name="simpleType" type="xs:localSimpleType"/>
              <xs:element name="complexType" type="xs:localComplexType"/>
            </xs:choice>
            <xs:element name="alternative" type="xs:altType"
                      minOccurs="0" maxOccurs="unbounded"/>
            <xs:group ref="xs:identityConstraint" minOccurs="0"
                      maxOccurs="unbounded"/>
          </xs:sequence>
          <xs:attribute name="ref" use="prohibited"/>
          <xs:attribute name="form" use="prohibited"/>
          <xs:attribute name="targetNamespace" use="prohibited"/>
          <xs:attribute name="minOccurs" use="prohibited"/>
          <xs:attribute name="maxOccurs" use="prohibited"/>
          <xs:attribute name="name" type="xs:NCName" use="required"/>
          <xs:anyAttribute namespace="##other" processContents="lax"/>
        </xs:restriction>
      </xs:complexContent>
    </xs:complexType>
    "###
    .trim();

    let open_attrs: xs::TopLevelComplexType = xmlity_quick_xml::de::from_str(open_attrs).unwrap();
    let annotated: xs::TopLevelComplexType = xmlity_quick_xml::de::from_str(annotated).unwrap();
    let element: xs::TopLevelComplexType = xmlity_quick_xml::de::from_str(element).unwrap();
    let top_level_element: xs::TopLevelComplexType =
        xmlity_quick_xml::de::from_str(top_level_element).unwrap();

    let namespace = XmlNamespace::XS;

    let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

    let _open_attrs = compiled_namespace
        .import_top_level_complex_type(&open_attrs)
        .unwrap()
        .into_owned();

    let _annotated = compiled_namespace
        .import_top_level_complex_type(&annotated)
        .unwrap()
        .into_owned();

    let _element = compiled_namespace
        .import_top_level_complex_type(&element)
        .unwrap()
        .into_owned();

    let top_level_element = compiled_namespace
        .import_top_level_complex_type(&top_level_element)
        .unwrap()
        .into_owned();

    let mut context = XmlnsContext::new();

    context.add_namespace(compiled_namespace);

    loop {
        let mut total_change = TransformChange::Unchanged;

        total_change |= context
            .transform(&namespace, ExpandAttributeGroups::new())
            .unwrap();

        // total_change |= context.transform(&namespace, ExpandGroups::new()).unwrap();

        total_change |= context
            .transform(&namespace, ExpandExtensionFragments::new())
            .unwrap();

        total_change |= context
            .transform(&namespace, ExpandRestrictionFragments::new())
            .unwrap();

        total_change |= context
            .transform(&namespace, RemoveProhibitedAttributes::new())
            .unwrap();

        if total_change == TransformChange::Unchanged {
            break;
        }
    }

    let mut generator = Generator::new(&context);

    generator.bind_namespace(namespace.clone(), parse_quote!(crate::xmlns));

    generator.bind_types(xsd_codegen_xmlity::binds::StdXsdTypes);

    generator.bind_type(
        ExpandedName::new(
            LocalName::new_dangerous("derivationSet"),
            Some(XmlNamespace::XS),
        ),
        BoundType {
            ty: TypeReference::new_static(parse_quote!(crate::xmlns::DerivationSet)),
            ty_type: TypeType::Simple,
            serialize_with: None,
            deserialize_with: None,
        },
    );

    generator.bind_type(
        ExpandedName::new(LocalName::new_dangerous("blockSet"), Some(XmlNamespace::XS)),
        BoundType {
            ty: TypeReference::new_static(parse_quote!(crate::xmlns::BlockSet)),
            ty_type: TypeType::Simple,
            serialize_with: None,
            deserialize_with: None,
        },
    );

    generator.bind_type(
        ExpandedName::new(
            LocalName::new_dangerous("formChoice"),
            Some(XmlNamespace::XS),
        ),
        BoundType {
            ty: TypeReference::new_static(parse_quote!(crate::xmlns::FormChoice)),
            ty_type: TypeType::Simple,
            serialize_with: None,
            deserialize_with: None,
        },
    );

    let mut actual_items = Vec::new();

    let (_, local_items) = generator
        .generate_top_level_type(&top_level_element)
        .unwrap();

    actual_items.extend(local_items);

    let print_code = actual_items
        .iter()
        .map(|item| item.to_token_stream())
        .collect::<proc_macro2::TokenStream>();

    println!(
        "{}",
        prettyplease::unparse(&syn::parse2(print_code).unwrap())
    );
}
