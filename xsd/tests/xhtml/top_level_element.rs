use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::schema as xs;
use xsd::schema_names as xsn;

const XHTML_HTML: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="html">
    <xs:complexType>
        <xs:sequence>
        <xs:element ref="head" />
        <xs:element ref="body" />
        </xs:sequence>
        <xs:attributeGroup ref="i18n" />
        <xs:attribute name="id" type="xs:ID" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_HEAD: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="head">
    <xs:annotation>
        <xs:documentation> content model is "head.misc" combined with a single title and an optional
        base element in any order </xs:documentation>
    </xs:annotation>
    <xs:complexType>
        <xs:sequence>
        <xs:group ref="head.misc" />
        <xs:choice>
            <xs:sequence>
            <xs:element ref="title" />
            <xs:group ref="head.misc" />
            <xs:sequence minOccurs="0">
                <xs:element ref="base" />
                <xs:group ref="head.misc" />
            </xs:sequence>
            </xs:sequence>
            <xs:sequence>
            <xs:element ref="base" />
            <xs:group ref="head.misc" />
            <xs:element ref="title" />
            <xs:group ref="head.misc" />
            </xs:sequence>
        </xs:choice>
        </xs:sequence>
        <xs:attributeGroup ref="i18n" />
        <xs:attribute name="id" type="xs:ID" />
        <xs:attribute name="profile" type="URI" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_TITLE: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="title">
    <xs:annotation>
        <xs:documentation> The title element is not considered part of the flow of text. It should be
        displayed, for example as the page header or window title. Exactly one title is required per
        document. </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:attributeGroup ref="i18n" />
        <xs:attribute name="id" type="xs:ID" />
    </xs:complexType>
</xs:element>
"###;

fn xhtml_title() -> xs::TopLevelElement {
    xs::TopLevelElement(
        xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("title"))
            .annotation(
                xs::Annotation::builder()
                    .content(vec![xs::Documentation::builder()
                        .any(vec![xmlity::xml!(
                    r###" The title element is not considered part of the flow of text. It should be
        displayed, for example as the page header or window title. Exactly one title is required per
        document. "###
                ).into()])
                        .build()
                        .into()])
                    .build(),
            )
            .child_1(
                xs::LocalComplexType::builder()
                    .mixed(true)
                    .content(
                        xs::ComplexTypeModel::Other {
                            open_content: None,
                            type_def_particle: None,
                            attr_decls: xs::AttrDecls::builder()
                                .declarations(bon::vec![
                                    xs::AttributeGroupRefType::builder()
                                        .ref_(xs::QName(ExpandedName::new(
                                            LocalName::new_dangerous("i18n"),
                                            Some(XmlNamespace::XHTML),
                                        )))
                                        .build(),
                                    xs::LocalAttribute::builder()
                                        .name(LocalName::new_dangerous("id"))
                                        .type_(xs::QName(xsn::ID.clone()))
                                        .build(),
                                ])
                                .build(),
                        }
                        .into(),
                    )
                    .build()
                    .into(),
            )
            .build(),
    )
}

const XHTML_BASE: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="base">
    <xs:annotation>
        <xs:documentation> document base URI </xs:documentation>
    </xs:annotation>
    <xs:complexType>
        <xs:attribute name="href" use="required" type="URI" />
        <xs:attribute name="id" type="xs:ID" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_META: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="meta">
    <xs:annotation>
        <xs:documentation> generic metainformation </xs:documentation>
    </xs:annotation>
    <xs:complexType>
        <xs:attributeGroup ref="i18n" />
        <xs:attribute name="id" type="xs:ID" />
        <xs:attribute name="http-equiv" />
        <xs:attribute name="name" />
        <xs:attribute name="content" use="required" />
        <xs:attribute name="scheme" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_LINK: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="link">
    <xs:annotation>
        <xs:documentation> Relationship values can be used in principle: a) for document specific
        toolbars/menus when used with the link element in document head e.g. start, contents,
        previous, next, index, end, help b) to link to a separate style sheet (rel="stylesheet") c)
        to make a link to a script (rel="script") d) by stylesheets to control how collections of
        html nodes are rendered into printed documents e) to make a link to a printable version of
        this document e.g. a PostScript or PDF version (rel="alternate" media="print") </xs:documentation>
    </xs:annotation>
    <xs:complexType>
        <xs:attributeGroup ref="attrs" />
        <xs:attribute name="charset" type="Charset" />
        <xs:attribute name="href" type="URI" />
        <xs:attribute name="hreflang" type="LanguageCode" />
        <xs:attribute name="type" type="ContentType" />
        <xs:attribute name="rel" type="LinkTypes" />
        <xs:attribute name="rev" type="LinkTypes" />
        <xs:attribute name="media" type="MediaDesc" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_STYLE: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="style">
    <xs:annotation>
        <xs:documentation> style info, which may include CDATA sections </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:attributeGroup ref="i18n" />
        <xs:attribute name="id" type="xs:ID" />
        <xs:attribute name="type" use="required" type="ContentType" />
        <xs:attribute name="media" type="MediaDesc" />
        <xs:attribute name="title" type="Text" />
        <xs:attribute ref="xml:space" fixed="preserve" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_SCRIPT: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="script">
    <xs:annotation>
        <xs:documentation> script statements, which may include CDATA sections </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:attribute name="id" type="xs:ID" />
        <xs:attribute name="charset" type="Charset" />
        <xs:attribute name="type" use="required" type="ContentType" />
        <xs:attribute name="src" type="URI" />
        <xs:attribute name="defer">
            <xs:simpleType>
                <xs:restriction base="xs:token">
                <xs:enumeration value="defer" />
                </xs:restriction>
            </xs:simpleType>
        </xs:attribute>
        <xs:attribute ref="xml:space" fixed="preserve" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_NOSCRIPT: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="noscript">
    <xs:annotation>
        <xs:documentation> alternate content container for non script-based rendering </xs:documentation>
    </xs:annotation>
    <xs:complexType>
        <xs:complexContent>
            <xs:extension base="Block">
                <xs:attributeGroup ref="attrs" />
            </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_BODY: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="body">
    <xs:complexType>
        <xs:complexContent>
            <xs:extension base="Block">
                <xs:attributeGroup ref="attrs" />
                <xs:attribute name="onload" type="Script" />
                <xs:attribute name="onunload" type="Script" />
            </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_DIV: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="div">
    <xs:annotation>
        <xs:documentation> generic language/style container </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
            <xs:extension base="Flow">
                <xs:attributeGroup ref="attrs" />
            </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_P: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="p">
    <xs:complexType mixed="true">
        <xs:complexContent>
            <xs:extension base="Inline">
                <xs:attributeGroup ref="attrs" />
            </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_H1: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="h1">
    <xs:complexType mixed="true">
        <xs:complexContent>
            <xs:extension base="Inline">
                <xs:attributeGroup ref="attrs" />
            </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_H2: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="h2">
    <xs:complexType mixed="true">
        <xs:complexContent>
            <xs:extension base="Inline">
                <xs:attributeGroup ref="attrs" />
            </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_H3: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="h3">
    <xs:complexType mixed="true">
        <xs:complexContent>
            <xs:extension base="Inline">
                <xs:attributeGroup ref="attrs" />
            </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_H4: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="h4">
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_H5: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="h5">
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_H6: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="h6">
    <xs:complexType mixed="true">
        <xs:complexContent>
            <xs:extension base="Inline">
                <xs:attributeGroup ref="attrs" />
            </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_UL: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="ul">
    <xs:annotation>
        <xs:documentation> Unordered list </xs:documentation>
    </xs:annotation>
    <xs:complexType>
        <xs:sequence>
            <xs:element maxOccurs="unbounded" ref="li" />
        </xs:sequence>
        <xs:attributeGroup ref="attrs" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_OL: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="ol">
    <xs:annotation>
        <xs:documentation> Ordered (numbered) list </xs:documentation>
    </xs:annotation>
    <xs:complexType>
        <xs:sequence>
        <xs:element maxOccurs="unbounded" ref="li" />
        </xs:sequence>
        <xs:attributeGroup ref="attrs" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_LI: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="li">
    <xs:annotation>
        <xs:documentation> list item </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Flow">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_DL: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="dl">
    <xs:complexType>
        <xs:choice maxOccurs="unbounded">
        <xs:element ref="dt" />
        <xs:element ref="dd" />
        </xs:choice>
        <xs:attributeGroup ref="attrs" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_DT: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="dt">
    <xs:complexType mixed="true">
        <xs:complexContent>
            <xs:extension base="Inline">
                <xs:attributeGroup ref="attrs" />
            </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_DD: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="dd">
    <xs:complexType mixed="true">
        <xs:complexContent>
            <xs:extension base="Flow">
                <xs:attributeGroup ref="attrs" />
            </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_ADDRESS: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="address">
    <xs:annotation>
        <xs:documentation> information on author </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
            <xs:extension base="Inline">
                <xs:attributeGroup ref="attrs" />
            </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_HR: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="hr">
    <xs:complexType>
        <xs:attributeGroup ref="attrs" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_PRE: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="pre">
    <xs:annotation>
        <xs:documentation> content is "Inline" excluding "img|object|big|small|sub|sup" </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="pre.content">
            <xs:attributeGroup ref="attrs" />
            <xs:attribute ref="xml:space" fixed="preserve" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_BLOCKQUOTE: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="blockquote">
    <xs:complexType>
        <xs:complexContent>
        <xs:extension base="Block">
            <xs:attributeGroup ref="attrs" />
            <xs:attribute name="cite" type="URI" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_INS: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="ins">
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Flow">
            <xs:attributeGroup ref="attrs" />
            <xs:attribute name="cite" type="URI" />
            <xs:attribute name="datetime" type="Datetime" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_DEL: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="del">
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Flow">
            <xs:attributeGroup ref="attrs" />
            <xs:attribute name="cite" type="URI" />
            <xs:attribute name="datetime" type="Datetime" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_A: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="a">
    <xs:annotation>
        <xs:documentation> content is "Inline" except that anchors shouldn't be nested </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="a.content">
            <xs:attributeGroup ref="attrs" />
            <xs:attributeGroup ref="focus" />
            <xs:attribute name="charset" type="Charset" />
            <xs:attribute name="type" type="ContentType" />
            <xs:attribute name="name" type="xs:NMTOKEN" />
            <xs:attribute name="href" type="URI" />
            <xs:attribute name="hreflang" type="LanguageCode" />
            <xs:attribute name="rel" type="LinkTypes" />
            <xs:attribute name="rev" type="LinkTypes" />
            <xs:attribute name="shape" default="rect" type="Shape" />
            <xs:attribute name="coords" type="Coords" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_SPAN: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="span">
    <xs:annotation>
        <xs:documentation> generic language/style container </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_BDO: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="bdo">
    <xs:annotation>
        <xs:documentation> I18N BiDi over-ride </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="coreattrs" />
            <xs:attributeGroup ref="events" />
            <xs:attribute name="lang" type="LanguageCode" />
            <xs:attribute ref="xml:lang" />
            <xs:attribute name="dir" use="required">
            <xs:simpleType>
                <xs:restriction base="xs:token">
                <xs:enumeration value="ltr" />
                <xs:enumeration value="rtl" />
                </xs:restriction>
            </xs:simpleType>
            </xs:attribute>
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_BR: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="br">
    <xs:annotation>
        <xs:documentation> forced line break </xs:documentation>
    </xs:annotation>
    <xs:complexType>
        <xs:attributeGroup ref="coreattrs" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_EM: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="em">
    <xs:annotation>
        <xs:documentation> emphasis </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_STRONG: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="strong">
    <xs:annotation>
        <xs:documentation> strong emphasis </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_DFN: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="dfn">
    <xs:annotation>
        <xs:documentation> definitional </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_CODE: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="code">
    <xs:annotation>
        <xs:documentation> program code </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_SAMP: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="samp">
    <xs:annotation>
        <xs:documentation> sample </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_KBD: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="kbd">
    <xs:annotation>
        <xs:documentation> something user would type </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_VAR: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="var">
    <xs:annotation>
        <xs:documentation> variable </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_CITE: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="cite">
    <xs:annotation>
        <xs:documentation> citation </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_ABBR: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="abbr">
    <xs:annotation>
        <xs:documentation> abbreviation </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_ACRONYM: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="acronym">
    <xs:annotation>
        <xs:documentation> acronym </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_Q: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="q">
    <xs:annotation>
        <xs:documentation> inlined quote </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
            <xs:attribute name="cite" type="URI" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_SUB: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="sub">
    <xs:annotation>
        <xs:documentation> subscript </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_SUP: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="sup">
    <xs:annotation>
        <xs:documentation> superscript </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_TT: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="tt">
    <xs:annotation>
        <xs:documentation> fixed pitch font </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_I: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="i">
    <xs:annotation>
        <xs:documentation> italic font </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_B: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="b">
    <xs:annotation>
        <xs:documentation> bold font </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_BIG: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="big">
    <xs:annotation>
        <xs:documentation> bigger font </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_SMALL: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="small">
    <xs:annotation>
        <xs:documentation> smaller font </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_OBJECT: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="object">
    <xs:complexType mixed="true">
        <xs:choice minOccurs="0" maxOccurs="unbounded">
        <xs:element ref="param" />
        <xs:group ref="block" />
        <xs:element ref="form" />
        <xs:group ref="inline" />
        <xs:group ref="misc" />
        </xs:choice>
        <xs:attributeGroup ref="attrs" />
        <xs:attribute name="declare">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="declare" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="classid" type="URI" />
        <xs:attribute name="codebase" type="URI" />
        <xs:attribute name="data" type="URI" />
        <xs:attribute name="type" type="ContentType" />
        <xs:attribute name="codetype" type="ContentType" />
        <xs:attribute name="archive" type="UriList" />
        <xs:attribute name="standby" type="Text" />
        <xs:attribute name="height" type="Length" />
        <xs:attribute name="width" type="Length" />
        <xs:attribute name="usemap" type="URI" />
        <xs:attribute name="name" type="xs:NMTOKEN" />
        <xs:attribute name="tabindex" type="tabindexNumber" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_PARAM: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="param">
    <xs:annotation>
        <xs:documentation> param is used to supply a named property value. In XML it would seem
        natural to follow RDF and support an abbreviated syntax where the param elements are
        replaced by attribute value pairs on the object start tag. </xs:documentation>
    </xs:annotation>
    <xs:complexType>
        <xs:attribute name="id" type="xs:ID" />
        <xs:attribute name="name" />
        <xs:attribute name="value" />
        <xs:attribute name="valuetype" default="data">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="data" />
            <xs:enumeration value="ref" />
            <xs:enumeration value="object" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="type" type="ContentType" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_IMG: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="img">
    <xs:complexType>
        <xs:attributeGroup ref="attrs" />
        <xs:attribute name="src" use="required" type="URI" />
        <xs:attribute name="alt" use="required" type="Text" />
        <xs:attribute name="longdesc" type="URI" />
        <xs:attribute name="height" type="Length" />
        <xs:attribute name="width" type="Length" />
        <xs:attribute name="usemap" type="URI">
        <xs:annotation>
            <xs:documentation> usemap points to a map element which may be in this document or an
            external document, although the latter is not widely supported </xs:documentation>
        </xs:annotation>
        </xs:attribute>
        <xs:attribute name="ismap">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="ismap" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
    </xs:complexType>
</xs:element>
"###;

const XHTML_MAP: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="map">
    <xs:complexType>
        <xs:choice>
        <xs:choice maxOccurs="unbounded">
            <xs:group ref="block" />
            <xs:element ref="form" />
            <xs:group ref="misc" />
        </xs:choice>
        <xs:element maxOccurs="unbounded" ref="area" />
        </xs:choice>
        <xs:attributeGroup ref="i18n" />
        <xs:attributeGroup ref="events" />
        <xs:attribute name="id" use="required" type="xs:ID" />
        <xs:attribute name="class" />
        <xs:attribute name="style" type="StyleSheet" />
        <xs:attribute name="title" type="Text" />
        <xs:attribute name="name" type="xs:NMTOKEN" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_AREA: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="area">
    <xs:complexType>
        <xs:attributeGroup ref="attrs" />
        <xs:attributeGroup ref="focus" />
        <xs:attribute name="shape" default="rect" type="Shape" />
        <xs:attribute name="coords" type="Coords" />
        <xs:attribute name="href" type="URI" />
        <xs:attribute name="nohref">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="nohref" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="alt" use="required" type="Text" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_FORM: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="form">
    <xs:complexType>
        <xs:complexContent>
        <xs:extension base="form.content">
            <xs:attributeGroup ref="attrs" />
            <xs:attribute name="action" use="required" type="URI" />
            <xs:attribute name="method" default="get">
            <xs:simpleType>
                <xs:restriction base="xs:token">
                <xs:enumeration value="get" />
                <xs:enumeration value="post" />
                </xs:restriction>
            </xs:simpleType>
            </xs:attribute>
            <xs:attribute name="enctype" type="ContentType"
            default="application/x-www-form-urlencoded" />
            <xs:attribute name="onsubmit" type="Script" />
            <xs:attribute name="onreset" type="Script" />
            <xs:attribute name="accept" type="ContentTypes" />
            <xs:attribute name="accept-charset" type="Charsets" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_LABEL: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="label">
    <xs:annotation>
        <xs:documentation> Each label must not contain more than ONE field Label elements shouldn't be
        nested. </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
            <xs:attribute name="for" type="xs:IDREF" />
            <xs:attribute name="accesskey" type="Character" />
            <xs:attribute name="onfocus" type="Script" />
            <xs:attribute name="onblur" type="Script" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_INPUT: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="input">
    <xs:annotation>
        <xs:documentation> form control </xs:documentation>
    </xs:annotation>
    <xs:complexType>
        <xs:attributeGroup ref="attrs" />
        <xs:attributeGroup ref="focus" />
        <xs:attribute name="type" default="text" type="InputType" />
        <xs:attribute name="name">
        <xs:annotation>
            <xs:documentation> the name attribute is required for all but submit &amp; reset </xs:documentation>
        </xs:annotation>
        </xs:attribute>
        <xs:attribute name="value" />
        <xs:attribute name="checked">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="checked" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="disabled">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="disabled" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="readonly">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="readonly" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="size" />
        <xs:attribute name="maxlength" type="Number" />
        <xs:attribute name="src" type="URI" />
        <xs:attribute name="alt" />
        <xs:attribute name="usemap" type="URI" />
        <xs:attribute name="onselect" type="Script" />
        <xs:attribute name="onchange" type="Script" />
        <xs:attribute name="accept" type="ContentTypes" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_SELECT: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="select">
    <xs:annotation>
        <xs:documentation> option selector </xs:documentation>
    </xs:annotation>
    <xs:complexType>
        <xs:choice maxOccurs="unbounded">
        <xs:element ref="optgroup" />
        <xs:element ref="option" />
        </xs:choice>
        <xs:attributeGroup ref="attrs" />
        <xs:attribute name="name" />
        <xs:attribute name="size" type="Number" />
        <xs:attribute name="multiple">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="multiple" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="disabled">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="disabled" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="tabindex" type="tabindexNumber" />
        <xs:attribute name="onfocus" type="Script" />
        <xs:attribute name="onblur" type="Script" />
        <xs:attribute name="onchange" type="Script" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_OPTGROUP: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="optgroup">
    <xs:annotation>
        <xs:documentation> option group </xs:documentation>
    </xs:annotation>
    <xs:complexType>
        <xs:sequence>
        <xs:element maxOccurs="unbounded" ref="option" />
        </xs:sequence>
        <xs:attributeGroup ref="attrs" />
        <xs:attribute name="disabled">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="disabled" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="label" use="required" type="Text" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_OPTION: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="option">
    <xs:annotation>
        <xs:documentation> selectable choice </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:attributeGroup ref="attrs" />
        <xs:attribute name="selected">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="selected" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="disabled">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="disabled" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="label" type="Text" />
        <xs:attribute name="value" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_TEXTAREA: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="textarea">
    <xs:annotation>
        <xs:documentation> multi-line text field </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:attributeGroup ref="attrs" />
        <xs:attributeGroup ref="focus" />
        <xs:attribute name="name" />
        <xs:attribute name="rows" use="required" type="Number" />
        <xs:attribute name="cols" use="required" type="Number" />
        <xs:attribute name="disabled">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="disabled" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="readonly">
        <xs:simpleType>
            <xs:restriction base="xs:token">
            <xs:enumeration value="readonly" />
            </xs:restriction>
        </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="onselect" type="Script" />
        <xs:attribute name="onchange" type="Script" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_FIELDSET: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="fieldset">
    <xs:annotation>
        <xs:documentation> The fieldset element is used to group form fields. Only one legend element
        should occur in the content and if present should only be preceded by whitespace. NOTE: this
        content model is different from the XHTML 1.0 DTD, closer to the intended content model in
        HTML4 DTD </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:sequence>
        <xs:element ref="legend" />
        <xs:choice minOccurs="0" maxOccurs="unbounded">
            <xs:group ref="block" />
            <xs:element ref="form" />
            <xs:group ref="inline" />
            <xs:group ref="misc" />
        </xs:choice>
        </xs:sequence>
        <xs:attributeGroup ref="attrs" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_LEGEND: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="legend">
    <xs:annotation>
        <xs:documentation> fieldset label </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Inline">
            <xs:attributeGroup ref="attrs" />
            <xs:attribute name="accesskey" type="Character" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_BUTTON: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="button">
    <xs:annotation>
        <xs:documentation> Content is "Flow" excluding a, form and form controls </xs:documentation>
    </xs:annotation>
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="button.content">
            <xs:attributeGroup ref="attrs" />
            <xs:attributeGroup ref="focus" />
            <xs:attribute name="name" />
            <xs:attribute name="value" />
            <xs:attribute name="type" default="submit">
            <xs:simpleType>
                <xs:restriction base="xs:token">
                <xs:enumeration value="button" />
                <xs:enumeration value="submit" />
                <xs:enumeration value="reset" />
                </xs:restriction>
            </xs:simpleType>
            </xs:attribute>
            <xs:attribute name="disabled">
            <xs:simpleType>
                <xs:restriction base="xs:token">
                <xs:enumeration value="disabled" />
                </xs:restriction>
            </xs:simpleType>
            </xs:attribute>
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_TABLE: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="table">
    <xs:complexType>
        <xs:sequence>
        <xs:element minOccurs="0" ref="caption" />
        <xs:choice>
            <xs:element minOccurs="0" maxOccurs="unbounded" ref="col" />
            <xs:element minOccurs="0" maxOccurs="unbounded" ref="colgroup" />
        </xs:choice>
        <xs:element minOccurs="0" ref="thead" />
        <xs:element minOccurs="0" ref="tfoot" />
        <xs:choice>
            <xs:element maxOccurs="unbounded" ref="tbody" />
            <xs:element maxOccurs="unbounded" ref="tr" />
        </xs:choice>
        </xs:sequence>
        <xs:attributeGroup ref="attrs" />
        <xs:attribute name="summary" type="Text" />
        <xs:attribute name="width" type="Length" />
        <xs:attribute name="border" type="Pixels" />
        <xs:attribute name="frame" type="TFrame" />
        <xs:attribute name="rules" type="TRules" />
        <xs:attribute name="cellspacing" type="Length" />
        <xs:attribute name="cellpadding" type="Length" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_CAPTION: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="caption">
<xs:complexType mixed="true">
    <xs:complexContent>
    <xs:extension base="Inline">
        <xs:attributeGroup ref="attrs" />
    </xs:extension>
    </xs:complexContent>
</xs:complexType>
</xs:element>
"###;

const XHTML_THEAD: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="thead">
  <xs:complexType>
    <xs:sequence>
      <xs:element maxOccurs="unbounded" ref="tr"/>
    </xs:sequence>
    <xs:attributeGroup ref="attrs"/>
    <xs:attributeGroup ref="cellhalign"/>
    <xs:attributeGroup ref="cellvalign"/>
  </xs:complexType>
</xs:element>
"###;

fn xhtml_thead() -> xs::TopLevelElement {
    xs::TopLevelElement(
        xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("thead"))
            .child_1(
                xs::LocalComplexType::builder()
                    .content(xs::ComplexTypeModel::Other {
                        open_content: None,
                        type_def_particle: Some(
                            xs::SequenceType::builder()
                                .content(vec![xs::LocalElement::builder()
                                    .ref_(xs::QName(ExpandedName::new(
                                        LocalName::new_dangerous("tr"),
                                        Some(XmlNamespace::XHTML),
                                    )))
                                    .max_occurs(xs::MaxOccurs(xs::MaxOccursValue::Unbounded))
                                    .build()
                                    .into()])
                                .build()
                                .into(),
                        ),
                        attr_decls: xs::AttrDecls::builder()
                            .declarations(vec![
                                xs::AttributeGroupRefType::builder()
                                    .ref_(xs::QName(ExpandedName::new(
                                        LocalName::new_dangerous("attrs"),
                                        Some(XmlNamespace::XHTML),
                                    )))
                                    .build()
                                    .into(),
                                xs::AttributeGroupRefType::builder()
                                    .ref_(xs::QName(ExpandedName::new(
                                        LocalName::new_dangerous("cellhalign"),
                                        Some(XmlNamespace::XHTML),
                                    )))
                                    .build()
                                    .into(),
                                xs::AttributeGroupRefType::builder()
                                    .ref_(xs::QName(ExpandedName::new(
                                        LocalName::new_dangerous("cellvalign"),
                                        Some(XmlNamespace::XHTML),
                                    )))
                                    .build()
                                    .into(),
                            ])
                            .build(),
                    })
                    .build()
                    .into(),
            )
            .build(),
    )
}

const XHTML_TFOOT: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="tfoot">
<xs:complexType>
    <xs:sequence>
    <xs:element maxOccurs="unbounded" ref="tr" />
    </xs:sequence>
    <xs:attributeGroup ref="attrs" />
    <xs:attributeGroup ref="cellhalign" />
    <xs:attributeGroup ref="cellvalign" />
</xs:complexType>
</xs:element>
"###;

const XHTML_TBODY: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="tbody">
<xs:complexType>
    <xs:sequence>
    <xs:element maxOccurs="unbounded" ref="tr" />
    </xs:sequence>
    <xs:attributeGroup ref="attrs" />
    <xs:attributeGroup ref="cellhalign" />
    <xs:attributeGroup ref="cellvalign" />
</xs:complexType>
</xs:element>
"###;

const XHTML_COLGROUP: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="colgroup">
<xs:annotation>
    <xs:documentation> colgroup groups a set of col elements. It allows you to group several
    semantically related columns together. </xs:documentation>
</xs:annotation>
<xs:complexType>
    <xs:sequence>
    <xs:element minOccurs="0" maxOccurs="unbounded" ref="col" />
    </xs:sequence>
    <xs:attributeGroup ref="attrs" />
    <xs:attribute name="span" default="1" type="Number" />
    <xs:attribute name="width" type="MultiLength" />
    <xs:attributeGroup ref="cellhalign" />
    <xs:attributeGroup ref="cellvalign" />
</xs:complexType>
</xs:element>
"###;

const XHTML_COL: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="col">
<xs:annotation>
    <xs:documentation> col elements define the alignment properties for cells in one or more
    columns. The width attribute specifies the width of the columns, e.g. width=64 width in
    screen pixels width=0.5* relative width of 0.5 The span attribute causes the attributes of
    one col element to apply to more than one column. </xs:documentation>
</xs:annotation>
<xs:complexType>
    <xs:attributeGroup ref="attrs" />
    <xs:attribute name="span" default="1" type="Number" />
    <xs:attribute name="width" type="MultiLength" />
    <xs:attributeGroup ref="cellhalign" />
    <xs:attributeGroup ref="cellvalign" />
</xs:complexType>
</xs:element>
"###;

const XHTML_TR: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="tr">
    <xs:complexType>
        <xs:choice maxOccurs="unbounded">
            <xs:element ref="th" />
            <xs:element ref="td" />
        </xs:choice>
        <xs:attributeGroup ref="attrs" />
        <xs:attributeGroup ref="cellhalign" />
        <xs:attributeGroup ref="cellvalign" />
    </xs:complexType>
</xs:element>
"###;

const XHTML_TH: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="th">
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Flow">
            <xs:attributeGroup ref="attrs" />
            <xs:attribute name="abbr" type="Text" />
            <xs:attribute name="axis" />
            <xs:attribute name="headers" type="xs:IDREFS" />
            <xs:attribute name="scope" type="Scope" />
            <xs:attribute name="rowspan" default="1" type="Number" />
            <xs:attribute name="colspan" default="1" type="Number" />
            <xs:attributeGroup ref="cellhalign" />
            <xs:attributeGroup ref="cellvalign" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

const XHTML_TD: &str = r###"
<xs:element xmlns="http://www.w3.org/1999/xhtml" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xml="http://www.w3.org/XML/1998/namespace" name="td">
    <xs:complexType mixed="true">
        <xs:complexContent>
        <xs:extension base="Flow">
            <xs:attributeGroup ref="attrs" />
            <xs:attribute name="abbr" type="Text" />
            <xs:attribute name="axis" />
            <xs:attribute name="headers" type="xs:IDREFS" />
            <xs:attribute name="scope" type="Scope" />
            <xs:attribute name="rowspan" default="1" type="Number" />
            <xs:attribute name="colspan" default="1" type="Number" />
            <xs:attributeGroup ref="cellhalign" />
            <xs:attributeGroup ref="cellvalign" />
        </xs:extension>
        </xs:complexContent>
    </xs:complexType>
</xs:element>
"###;

#[rstest::rstest]
#[case::html(XHTML_HTML, None)]
#[case::head(XHTML_HEAD, None)]
#[case::title(XHTML_TITLE, Some(xhtml_title()))]
#[case::base(XHTML_BASE, None)]
#[case::meta(XHTML_META, None)]
#[case::link(XHTML_LINK, None)]
#[case::style(XHTML_STYLE, None)]
#[case::script(XHTML_SCRIPT, None)]
#[case::noscript(XHTML_NOSCRIPT, None)]
#[case::body(XHTML_BODY, None)]
#[case::div(XHTML_DIV, None)]
#[case::p(XHTML_P, None)]
#[case::h1(XHTML_H1, None)]
#[case::h2(XHTML_H2, None)]
#[case::h3(XHTML_H3, None)]
#[case::h4(XHTML_H4, None)]
#[case::h5(XHTML_H5, None)]
#[case::h6(XHTML_H6, None)]
#[case::ul(XHTML_UL, None)]
#[case::ol(XHTML_OL, None)]
#[case::li(XHTML_LI, None)]
#[case::dl(XHTML_DL, None)]
#[case::dt(XHTML_DT, None)]
#[case::dd(XHTML_DD, None)]
#[case::address(XHTML_ADDRESS, None)]
#[case::hr(XHTML_HR, None)]
#[case::pre(XHTML_PRE, None)]
#[case::blockquote(XHTML_BLOCKQUOTE, None)]
#[case::ins(XHTML_INS, None)]
#[case::del(XHTML_DEL, None)]
#[case::a(XHTML_A, None)]
#[case::span(XHTML_SPAN, None)]
#[case::bdo(XHTML_BDO, None)]
#[case::br(XHTML_BR, None)]
#[case::em(XHTML_EM, None)]
#[case::strong(XHTML_STRONG, None)]
#[case::dfn(XHTML_DFN, None)]
#[case::code(XHTML_CODE, None)]
#[case::samp(XHTML_SAMP, None)]
#[case::kbd(XHTML_KBD, None)]
#[case::var(XHTML_VAR, None)]
#[case::cite(XHTML_CITE, None)]
#[case::abbr(XHTML_ABBR, None)]
#[case::acronym(XHTML_ACRONYM, None)]
#[case::q(XHTML_Q, None)]
#[case::sub(XHTML_SUB, None)]
#[case::sup(XHTML_SUP, None)]
#[case::tt(XHTML_TT, None)]
#[case::i(XHTML_I, None)]
#[case::b(XHTML_B, None)]
#[case::big(XHTML_BIG, None)]
#[case::small(XHTML_SMALL, None)]
#[case::object(XHTML_OBJECT, None)]
#[case::param(XHTML_PARAM, None)]
#[case::img(XHTML_IMG, None)]
#[case::map(XHTML_MAP, None)]
#[case::area(XHTML_AREA, None)]
#[case::form(XHTML_FORM, None)]
#[case::label(XHTML_LABEL, None)]
#[case::input(XHTML_INPUT, None)]
#[case::select(XHTML_SELECT, None)]
#[case::optgroup(XHTML_OPTGROUP, None)]
#[case::option(XHTML_OPTION, None)]
#[case::textarea(XHTML_TEXTAREA, None)]
#[case::fieldset(XHTML_FIELDSET, None)]
#[case::legend(XHTML_LEGEND, None)]
#[case::button(XHTML_BUTTON, None)]
#[case::table(XHTML_TABLE, None)]
#[case::caption(XHTML_CAPTION, None)]
#[case::thead(XHTML_THEAD, Some(xhtml_thead()))]
#[case::tfoot(XHTML_TFOOT, None)]
#[case::tbody(XHTML_TBODY, None)]
#[case::colgroup(XHTML_COLGROUP, None)]
#[case::col(XHTML_COL, None)]
#[case::tr(XHTML_TR, None)]
#[case::th(XHTML_TH, None)]
#[case::td(XHTML_TD, None)]
fn deserialize(#[case] xml: &str, #[case] expected: Option<xs::TopLevelElement>) {
    let xml = xml.trim();
    let element: xs::TopLevelElement = xmlity_quick_xml::de::from_str(xml).unwrap();

    if let Some(expected) = expected {
        pretty_assertions::assert_eq!(element, expected);
    }
}
