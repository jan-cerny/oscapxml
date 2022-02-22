use crate::utils::*;
use minidom::Element;

pub const XCCDF12_NS: &str = "http://checklists.nist.gov/xccdf/1.2";

#[derive(Debug)]
pub struct Benchmark {
    id: String,
    resolved: bool,
    style: Option<String>,
    style_href: Option<String>,
    statuses: Vec<Status>,
    titles: Vec<Title>,
    descriptions: Vec<Description>,
    notices: Vec<Notice>,
    front_matters: Vec<FrontMatter>,
    rear_matters: Vec<RearMatter>,
    references: Vec<Reference>,
    plain_texts: Vec<PlainText>,
    platform_specification: Option<PlatformSpecification>,
    platforms: Vec<Platform>,
    version: Version,
    metadata: Vec<Metadata>,
    models: Vec<Model>,
    profiles: Vec<Profile>,
    values: Vec<Value>,
    groups: Vec<Group>,
    rules: Vec<Rule>,
    test_results: Vec<TestResult>,
}

impl Benchmark {
    pub fn from_xml(benchmark_el: &Element) -> Result<Benchmark, String> {
        if !benchmark_el.is("Benchmark", XCCDF12_NS) {
            return Err(format!(
                "Unexpected element '{}', expected xccdf:Benchmark",
                benchmark_el.name()
            ));
        }
        let id = require_attr(benchmark_el, "id")?;
        let resolved = get_attr_default_bool(benchmark_el, "resolved", false)?;
        let style = get_attr(benchmark_el, "style");
        let style_href = get_attr(benchmark_el, "style-href");
        let mut statuses = Vec::new();
        let mut titles = Vec::new();
        let mut descriptions = Vec::new();
        let mut notices = Vec::new();
        let mut front_matters = Vec::new();
        let mut rear_matters = Vec::new();
        let mut references = Vec::new();
        let mut plain_texts = Vec::new();
        let mut platform_specification = None;
        let mut platforms = Vec::new();
        let mut version = None;
        let mut metadata = Vec::new();
        let mut models = Vec::new();
        let mut profiles = Vec::new();
        let mut values = Vec::new();
        let mut groups = Vec::new();
        let mut rules = Vec::new();
        let mut test_results = Vec::new();
        for child in benchmark_el.children() {
            match child.name() {
                "status" => statuses.push(Status::from_xml(child)?),
                "title" => titles.push(Title::from_xml(child)?),
                "description" => descriptions.push(Description::from_xml(child)?),
                "notice" => notices.push(Notice::from_xml(child)?),
                "front-matter" => front_matters.push(FrontMatter::from_xml(child)?),
                "rear-matter" => rear_matters.push(RearMatter::from_xml(child)?),
                "reference" => references.push(Reference::from_xml(child)?),
                "plain-text" => plain_texts.push(PlainText::from_xml(child)?),
                "platform-specification" => match platform_specification {
                    Some(_) => return Err(format!("Duplicate platform elements")),
                    None => platform_specification = Some(PlatformSpecification::from_xml(child)?),
                },
                "platform" => platforms.push(Platform::from_xml(child)?),
                "version" => match version {
                    Some(_) => return Err(format!("Duplicate version elements")),
                    None => version = Some(Version::from_xml(child)?),
                },
                "metadata" => metadata.push(Metadata::from_xml(child)?),
                "model" => models.push(Model::from_xml(child)?),
                "Profile" => profiles.push(Profile::from_xml(child)?),
                "Value" => values.push(Value::from_xml(child)?),
                "Group" => groups.push(Group::from_xml(child)?),
                "Rule" => rules.push(Rule::from_xml(child)?),
                "TestResult" => test_results.push(TestResult::from_xml(child)?),
                _ => {
                    return Err(format!("unexpected element {}", child.name()));
                }
            }
        }
        if statuses.len() == 0 {
            return Err(format!("xccdf:Benchmark {}: missing status element", id));
        }
        let version = match version {
            Some(x) => x,
            None => return Err(format!("xccdf:Benchmark {}: missing version element", id)),
        };
        Ok(Benchmark {
            id,
            resolved,
            style,
            style_href,
            statuses,
            titles,
            descriptions,
            notices,
            front_matters,
            rear_matters,
            references,
            plain_texts,
            platform_specification,
            platforms,
            version,
            metadata,
            models,
            profiles,
            values,
            groups,
            rules,
            test_results,
        })
    }

    pub fn print_information(&self) {
        println!("Benchmark ID: {}", self.id);
        if self.profiles.len() > 0 {
            println!("Profiles:");
            for profile in self.profiles.iter() {
                let title = match profile.titles.get(0) {
                    Some(t) => &t.title,
                    None => "Unknown",
                };
                let description = match profile.descriptions.get(0) {
                    Some(d) => &d.text,
                    None => "Unknown",
                };
                println!("* {}", title);
                println!("ID: {}", profile.id);
                println!("{}", description);
                println!();
            }
        }
    }
}

#[derive(Debug)]
struct Status {
    date: Option<String>,
    status: String,
}

impl Status {
    pub fn from_xml(el: &Element) -> Result<Status, String> {
        let date = get_attr(el, "date");
        let status = el.text();
        let allowed_statuses = vec!["incomplete", "draft", "interim", "accepted", "deprecated"];
        if !allowed_statuses.contains(&&status[..]) {
            return Err(format!("Unexpected xccdf:status value: '{}", status));
        }
        Ok(Status { date, status })
    }
}

#[derive(Debug)]
struct Title {
    title: String,
}

impl Title {
    pub fn from_xml(el: &Element) -> Result<Title, String> {
        let title = el.text();
        Ok(Title { title })
    }
}

#[derive(Debug)]
struct Description {
    text: String,
}

impl Description {
    pub fn from_xml(el: &Element) -> Result<Description, String> {
        let text = html_to_string(el);
        Ok(Description { text })
    }
}

#[derive(Debug)]
struct Notice {
    id: String,
    text: String,
}

impl Notice {
    pub fn from_xml(el: &Element) -> Result<Notice, String> {
        let id = require_attr(el, "id")?;
        let text = el.text();
        Ok(Notice { id, text })
    }
}

#[derive(Debug)]
struct FrontMatter {
    text: String,
}

impl FrontMatter {
    pub fn from_xml(el: &Element) -> Result<FrontMatter, String> {
        let text = el.text();
        Ok(FrontMatter { text })
    }
}

#[derive(Debug)]
struct RearMatter {
    text: String,
}

impl RearMatter {
    pub fn from_xml(el: &Element) -> Result<RearMatter, String> {
        let text = el.text();
        Ok(RearMatter { text })
    }
}

#[derive(Debug)]
struct Reference {
    text: String,
}

impl Reference {
    pub fn from_xml(el: &Element) -> Result<Reference, String> {
        let text = el.text();
        Ok(Reference { text })
    }
}

#[derive(Debug)]
struct PlainText {
    text: String,
}

impl PlainText {
    pub fn from_xml(el: &Element) -> Result<PlainText, String> {
        let text = el.text();
        Ok(PlainText { text })
    }
}

#[derive(Debug)]
struct PlatformSpecification {
    text: String,
}

impl PlatformSpecification {
    pub fn from_xml(el: &Element) -> Result<PlatformSpecification, String> {
        let text = el.text();
        Ok(PlatformSpecification { text })
    }
}

#[derive(Debug)]
struct Platform {
    idref: String,
}

impl Platform {
    pub fn from_xml(el: &Element) -> Result<Platform, String> {
        let idref = require_attr(el, "idref")?;
        Ok(Platform { idref })
    }
}

#[derive(Debug)]
struct Version {
    text: String,
}

impl Version {
    pub fn from_xml(el: &Element) -> Result<Version, String> {
        let text = el.text();
        Ok(Version { text })
    }
}

#[derive(Debug)]
struct Metadata {
    contributors: Vec<String>,
    publishers: Vec<String>,
    creators: Vec<String>,
    sources: Vec<String>,
}

impl Metadata {
    pub fn from_xml(el: &Element) -> Result<Metadata, String> {
        let mut contributors = Vec::new();
        let mut publishers = Vec::new();
        let mut creators = Vec::new();
        let mut sources = Vec::new();
        for child in el.children() {
            match child.name() {
                "contributor" => contributors.push(child.text()),
                "publisher" => publishers.push(child.text()),
                "creator" => creators.push(child.text()),
                "source" => sources.push(child.text()),
                _ => (),
            }
        }
        Ok(Metadata {
            contributors,
            publishers,
            creators,
            sources,
        })
    }
}

#[derive(Debug)]
struct Model {
    text: String,
}

impl Model {
    pub fn from_xml(el: &Element) -> Result<Model, String> {
        let text = el.text();
        Ok(Model { text })
    }
}

#[derive(Debug)]
struct Profile {
    // attributes
    id: String,
    prohibit_changes: bool,
    abstract_: bool,
    note_tag: Option<String>,
    extends: Option<String>,
    // child elements
    statuses: Vec<Status>,
    version: Option<Version>,
    titles: Vec<Title>,
    descriptions: Vec<Description>,
    references: Vec<Reference>,
    platforms: Vec<Platform>,
    selects: Vec<Select>,
    set_complex_values: Vec<SetComplexValue>,
    set_values: Vec<SetValue>,
    refine_values: Vec<RefineValue>,
    refine_rules: Vec<RefineRule>,
}

impl Profile {
    pub fn from_xml(el: &Element) -> Result<Profile, String> {
        let id = require_attr(el, "id")?;
        let prohibit_changes = get_attr_default_bool(el, "prohibitChanges", false)?;
        let abstract_ = get_attr_default(el, "abstract", false)?;
        let note_tag = get_attr(el, "note-tag");
        let extends = get_attr(el, "extends");
        let mut statuses = Vec::new();
        let mut version = None;
        let mut titles = Vec::new();
        let mut descriptions = Vec::new();
        let mut references = Vec::new();
        let mut platforms = Vec::new();
        let mut selects = Vec::new();
        let mut set_complex_values = Vec::new();
        let mut set_values = Vec::new();
        let mut refine_values = Vec::new();
        let mut refine_rules = Vec::new();
        for child in el.children() {
            match child.name() {
                "status" => statuses.push(Status::from_xml(child)?),
                "version" => match version {
                    Some(_) => return Err(format!("Duplicate version elements")),
                    None => version = Some(Version::from_xml(child)?),
                },
                "title" => titles.push(Title::from_xml(child)?),
                "description" => descriptions.push(Description::from_xml(child)?),
                "reference" => references.push(Reference::from_xml(child)?),
                "platform" => platforms.push(Platform::from_xml(child)?),
                "select" => selects.push(Select::from_xml(child)?),
                "set-complex-value" => set_complex_values.push(SetComplexValue::from_xml(child)?),
                "set-value" => set_values.push(SetValue::from_xml(child)?),
                "refine-value" => refine_values.push(RefineValue::from_xml(child)?),
                "refine-rule" => refine_rules.push(RefineRule::from_xml(child)?),
                _ => {
                    return Err(format!(
                        "Profile '{}': unexpected element '{}'",
                        id,
                        child.name()
                    ));
                }
            }
        }
        if titles.len() == 0 {
            return Err(format!("Profile '{}' doesn't have any title", id));
        }
        Ok(Profile {
            id,
            prohibit_changes,
            abstract_,
            note_tag,
            extends,
            statuses,
            version,
            titles,
            descriptions,
            references,
            platforms,
            selects,
            set_complex_values,
            set_values,
            refine_values,
            refine_rules,
        })
    }
}

#[derive(Debug)]
struct Value {
    id: String,
}

impl Value {
    pub fn from_xml(el: &Element) -> Result<Value, String> {
        let id = require_attr(el, "id")?;
        Ok(Value { id })
    }
}

#[derive(Debug)]
struct Group {
    // attributes
    id: String,
    abstract_: bool,
    cluster_id: Option<String>,
    extends: Option<String>,
    hidden: bool,
    prohibit_changes: bool,
    selected: bool,
    weight: f64,
    // children
    statuses: Vec<Status>,
    version: Option<Version>,
    titles: Vec<Title>,
    descriptions: Vec<Description>,
    warnings: Vec<Warning>,
    questions: Vec<Question>,
    references: Vec<Reference>,
    metadata: Vec<Metadata>,
    rationales: Vec<Rationale>,
    platforms: Vec<Platform>,
    requires: Vec<Requires>,
    conflicts: Vec<Conflicts>,
    values: Vec<Value>,
    groups: Vec<Group>,
    rules: Vec<Rule>,
}

impl Group {
    pub fn from_xml(el: &Element) -> Result<Group, String> {
        let id = require_attr(el, "id")?;
        let abstract_ = get_attr_default_bool(el, "abstract", false)?;
        let extends = get_attr(el, "extends");
        let hidden = get_attr_default_bool(el, "hidden", false)?;
        let prohibit_changes = get_attr_default_bool(el, "prohibitChanges", false)?;
        let selected = get_attr_default_bool(el, "selected", true)?;
        let weight = get_attr_default(el, "weight", 1.0)?;
        let cluster_id = get_attr(el, "cluster-id");

        let mut statuses = Vec::new();
        let mut version = None;
        let mut titles = Vec::new();
        let mut descriptions = Vec::new();
        let mut warnings = Vec::new();
        let mut questions = Vec::new();
        let mut references = Vec::new();
        let mut metadata = Vec::new();
        let mut rationales = Vec::new();
        let mut platforms = Vec::new();
        let mut requires = Vec::new();
        let mut conflicts = Vec::new();
        let mut values = Vec::new();
        let mut groups = Vec::new();
        let mut rules = Vec::new();

        for child in el.children() {
            match child.name() {
                "status" => statuses.push(Status::from_xml(child)?),
                "version" => match version {
                    Some(_) => return Err(format!("Duplicate version elements")),
                    None => version = Some(Version::from_xml(child)?),
                },
                "title" => titles.push(Title::from_xml(child)?),
                "description" => descriptions.push(Description::from_xml(child)?),
                "warning" => warnings.push(Warning::from_xml(child)?),
                "question" => questions.push(Question::from_xml(child)?),
                "reference" => references.push(Reference::from_xml(child)?),
                "metadata" => metadata.push(Metadata::from_xml(child)?),
                "rationale" => rationales.push(Rationale::from_xml(child)?),
                "platform" => platforms.push(Platform::from_xml(child)?),
                "requires" => requires.push(Requires::from_xml(child)?),
                "conflicts" => conflicts.push(Conflicts::from_xml(child)?),
                "Value" => values.push(Value::from_xml(child)?),
                "Group" => groups.push(Group::from_xml(child)?),
                "Rule" => rules.push(Rule::from_xml(child)?),
                _ => {
                    return Err(format!(
                        "Rule '{}': unexpected element '{}'",
                        id,
                        child.name()
                    ));
                }
            }
        }
        Ok(Group {
            id,
            abstract_,
            cluster_id,
            extends,
            hidden,
            prohibit_changes,
            selected,
            weight,
            statuses,
            version,
            titles,
            descriptions,
            warnings,
            questions,
            references,
            metadata,
            rationales,
            platforms,
            requires,
            conflicts,
            values,
            groups,
            rules,
        })
    }
}

#[derive(Debug)]
struct Rule {
    // attributes
    id: String,
    abstract_: bool,
    cluster_id: Option<String>,
    extends: Option<String>,
    hidden: bool,
    prohibit_changes: bool,
    selected: bool,
    weight: f64,
    role: String,
    severity: String,
    multiple: bool,
    // children
    statuses: Vec<Status>,
    version: Option<Version>,
    titles: Vec<Title>,
    descriptions: Vec<Description>,
    warnings: Vec<Warning>,
    questions: Vec<Question>,
    references: Vec<Reference>,
    metadata: Vec<Metadata>,
    rationales: Vec<Rationale>,
    platforms: Vec<Platform>,
    requires: Vec<Requires>,
    conflicts: Vec<Conflicts>,
    idents: Vec<Ident>,
    profile_notes: Vec<ProfileNote>,
    fixtexts: Vec<FixText>,
    fixes: Vec<Fix>,
    checks: Vec<Check>,
    complex_checks: Vec<ComplexCheck>,
}

impl Rule {
    pub fn from_xml(el: &Element) -> Result<Rule, String> {
        let id = require_attr(el, "id")?;
        let abstract_ = get_attr_default_bool(el, "abstract", false)?;
        let extends = get_attr(el, "extends");
        let hidden = get_attr_default_bool(el, "hidden", false)?;
        let prohibit_changes = get_attr_default_bool(el, "prohibitChanges", false)?;
        let selected = get_attr_default_bool(el, "selected", true)?;
        let weight = get_attr_default(el, "weight", 1.0)?;
        let cluster_id = get_attr(el, "cluster-id");
        let role = get_attr_default_options(
            el,
            "rule",
            String::from("full"),
            vec!["full", "unscored", "unchecked"],
        )?;
        let severity = get_attr_default_options(
            el,
            "severity",
            String::from("unknown"),
            vec!["unknown", "info", "low", "medium", "high"],
        )?;
        let multiple = get_attr_default_bool(el, "multiple", false)?;
        let mut statuses = Vec::new();
        let mut version = None;
        let mut titles = Vec::new();
        let mut descriptions = Vec::new();
        let mut warnings = Vec::new();
        let mut questions = Vec::new();
        let mut references = Vec::new();
        let mut metadata = Vec::new();
        let mut rationales = Vec::new();
        let mut platforms = Vec::new();
        let mut requires = Vec::new();
        let mut conflicts = Vec::new();
        let mut idents = Vec::new();
        let mut profile_notes = Vec::new();
        let mut fixtexts = Vec::new();
        let mut fixes = Vec::new();
        let mut checks = Vec::new();
        let mut complex_checks = Vec::new();
        for child in el.children() {
            match child.name() {
                "status" => statuses.push(Status::from_xml(child)?),
                "version" => match version {
                    Some(_) => return Err(format!("Duplicate version elements")),
                    None => version = Some(Version::from_xml(child)?),
                },
                "title" => titles.push(Title::from_xml(child)?),
                "description" => descriptions.push(Description::from_xml(child)?),
                "warning" => warnings.push(Warning::from_xml(child)?),
                "question" => questions.push(Question::from_xml(child)?),
                "reference" => references.push(Reference::from_xml(child)?),
                "metadata" => metadata.push(Metadata::from_xml(child)?),
                "rationale" => rationales.push(Rationale::from_xml(child)?),
                "platform" => platforms.push(Platform::from_xml(child)?),
                "requires" => requires.push(Requires::from_xml(child)?),
                "conflicts" => conflicts.push(Conflicts::from_xml(child)?),
                "ident" => idents.push(Ident::from_xml(child)?),
                "profile-note" => profile_notes.push(ProfileNote::from_xml(child)?),
                "fixtext" => fixtexts.push(FixText::from_xml(child)?),
                "fix" => fixes.push(Fix::from_xml(child)?),
                "check" => checks.push(Check::from_xml(child)?),
                "complex-check" => complex_checks.push(ComplexCheck::from_xml(child)?),
                _ => {
                    return Err(format!(
                        "Rule '{}': unexpected element '{}'",
                        id,
                        child.name()
                    ));
                }
            }
        }
        Ok(Rule {
            id,
            abstract_,
            cluster_id,
            extends,
            hidden,
            prohibit_changes,
            selected,
            weight,
            role,
            severity,
            multiple,
            statuses,
            version,
            titles,
            descriptions,
            warnings,
            questions,
            references,
            metadata,
            rationales,
            platforms,
            requires,
            conflicts,
            idents,
            profile_notes,
            fixtexts,
            fixes,
            checks,
            complex_checks,
        })
    }
}

#[derive(Debug)]
struct TestResult {
    id: String,
}

impl TestResult {
    pub fn from_xml(el: &Element) -> Result<TestResult, String> {
        let id = require_attr(el, "id")?;
        Ok(TestResult { id })
    }
}

#[derive(Debug)]
struct Select {
    idref: String,
    selected: bool,
}

impl Select {
    pub fn from_xml(el: &Element) -> Result<Select, String> {
        let idref = require_attr(el, "idref")?;
        let selected = require_attr(el, "selected")?.parse().unwrap();
        Ok(Select { idref, selected })
    }
}

#[derive(Debug)]
struct SetComplexValue {
    text: String,
}

impl SetComplexValue {
    pub fn from_xml(el: &Element) -> Result<SetComplexValue, String> {
        let text = el.text();
        Ok(SetComplexValue { text })
    }
}

#[derive(Debug)]
struct SetValue {
    text: String,
}

impl SetValue {
    pub fn from_xml(el: &Element) -> Result<SetValue, String> {
        let text = el.text();
        Ok(SetValue { text })
    }
}

#[derive(Debug)]
struct RefineValue {
    text: String,
}

impl RefineValue {
    pub fn from_xml(el: &Element) -> Result<RefineValue, String> {
        let text = el.text();
        Ok(RefineValue { text })
    }
}

#[derive(Debug)]
struct RefineRule {
    text: String,
}

impl RefineRule {
    pub fn from_xml(el: &Element) -> Result<RefineRule, String> {
        let text = el.text();
        Ok(RefineRule { text })
    }
}

#[derive(Debug)]
struct Warning {
    text: String,
}

impl Warning {
    pub fn from_xml(el: &Element) -> Result<Warning, String> {
        let text = el.text();
        Ok(Warning { text })
    }
}

#[derive(Debug)]
struct Question {
    text: String,
}

impl Question {
    pub fn from_xml(el: &Element) -> Result<Question, String> {
        let text = el.text();
        Ok(Question { text })
    }
}

#[derive(Debug)]
struct Rationale {
    text: String,
}

impl Rationale {
    pub fn from_xml(el: &Element) -> Result<Rationale, String> {
        let text = el.text();
        Ok(Rationale { text })
    }
}

#[derive(Debug)]
struct Requires {
    text: String,
}

impl Requires {
    pub fn from_xml(el: &Element) -> Result<Requires, String> {
        let text = el.text();
        Ok(Requires { text })
    }
}

#[derive(Debug)]
struct Conflicts {
    idref: String,
}

impl Conflicts {
    pub fn from_xml(el: &Element) -> Result<Conflicts, String> {
        let idref = require_attr(el, "idref")?;
        Ok(Conflicts { idref })
    }
}

#[derive(Debug)]
struct Ident {
    text: String,
    system: String,
}

impl Ident {
    pub fn from_xml(el: &Element) -> Result<Ident, String> {
        let text = el.text();
        let system = require_attr(el, "system")?;
        Ok(Ident { text, system })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident_from_xml_ok() {
        let e = Element::builder("ident", XCCDF12_NS)
            .attr("system", "https://gov.cz")
            .append("AC-24")
            .build();
        let real = Ident::from_xml(&e);
        assert!(real.is_ok());
        let real = real.unwrap();
        let expected = Ident {
            text: String::from("AC-24"),
            system: String::from("https://gov.cz"),
        };
        assert_eq!(real.system, expected.system);
        assert_eq!(real.text, expected.text);
    }

    #[test]
    fn test_ident_from_xml_err() {
        let f = Element::builder("ident", XCCDF12_NS)
            .attr("wrong_attribute_name", "https://gov.cz")
            .append("AC-24")
            .build();
        let real = Ident::from_xml(&f);
        assert!(real.is_err());
    }
}

#[derive(Debug)]
struct ProfileNote {
    text: String,
}

impl ProfileNote {
    pub fn from_xml(el: &Element) -> Result<ProfileNote, String> {
        let text = el.text();
        Ok(ProfileNote { text })
    }
}

#[derive(Debug)]
struct FixText {
    text: String,
}

impl FixText {
    pub fn from_xml(el: &Element) -> Result<FixText, String> {
        let text = el.text();
        Ok(FixText { text })
    }
}

#[derive(Debug)]
struct Fix {
    text: String,
}

impl Fix {
    pub fn from_xml(el: &Element) -> Result<Fix, String> {
        let text = el.text();
        Ok(Fix { text })
    }
}

#[derive(Debug)]
struct Check {
    text: String,
}

impl Check {
    pub fn from_xml(el: &Element) -> Result<Check, String> {
        let text = el.text();
        Ok(Check { text })
    }
}

#[derive(Debug)]
struct ComplexCheck {
    text: String,
}

impl ComplexCheck {
    pub fn from_xml(el: &Element) -> Result<ComplexCheck, String> {
        let text = el.text();
        Ok(ComplexCheck { text })
    }
}
