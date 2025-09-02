#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope {
    pub url: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiGroup {
    pub name: String,
    pub scopes: Vec<Scope>,
}

pub fn parse_info_file(content: &str) -> Vec<ApiGroup> {
    let mut api_groups: Vec<ApiGroup> = Vec::new();
    let mut current_api_group: Option<ApiGroup> = None;

    for line in content.lines() {
        if line.is_empty() {
            continue;
        }

        if line.ends_with(", v1") || line.ends_with(", v2") || line.ends_with(", v3") || line.ends_with(", v4") || line.ends_with(", v5") || line.ends_with(", v1beta1") || line.ends_with(", v2beta1") || line.ends_with(", v2beta") || line.ends_with(", v2alpha1") || line.ends_with(", v1b3") || line.ends_with(", v1management") || line.ends_with(", v1configuration") {
            // This is an API header
            if let Some(group) = current_api_group.take() {
                api_groups.push(group);
            }
            current_api_group = Some(ApiGroup {
                name: line.to_string(),
                scopes: Vec::new(),
            });
        } else if line == "Scopes" {
            // This line indicates the start of scopes for the current API group
            // We can ignore it, as the next lines will be the actual scopes
        } else {
            // This is a scope line
            if let Some(group) = current_api_group.as_mut() {
                let parts: Vec<&str> = line.splitn(2, '\t').collect();
                if parts.len() == 2 {
                    group.scopes.push(Scope {
                        url: parts[0].to_string(),
                        description: parts[1].to_string(),
                    });
                }
            }
        }
    }

    // Push the last API group if it exists
    if let Some(group) = current_api_group.take() {
        api_groups.push(group);
    }

    api_groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_info_file() {
        let content = r#"\
API One, v1
Scopes
https://www.googleapis.com/auth/scope1	Description for scope 1
https://www.googleapis.com/auth/scope2	Description for scope 2
API Two, v2
Scopes
https://www.googleapis.com/auth/scope3	Description for scope 3
"#;
        let api_groups = parse_info_file(content);

        assert_eq!(api_groups.len(), 2);

        assert_eq!(api_groups[0].name, "API One, v1");
        assert_eq!(api_groups[0].scopes.len(), 2);
        assert_eq!(api_groups[0].scopes[0].url, "https://www.googleapis.com/auth/scope1");
        assert_eq!(api_groups[0].scopes[0].description, "Description for scope 1");
        assert_eq!(api_groups[0].scopes[1].url, "https://www.googleapis.com/auth/scope2");
        assert_eq!(api_groups[0].scopes[1].description, "Description for scope 2");

        assert_eq!(api_groups[1].name, "API Two, v2");
        assert_eq!(api_groups[1].scopes.len(), 1);
        assert_eq!(api_groups[1].scopes[0].url, "https://www.googleapis.com/auth/scope3");
        assert_eq!(api_groups[1].scopes[0].description, "Description for scope 3");
    }

    #[test]
    fn test_parse_empty_content() {
        let content = "";
        let api_groups = parse_info_file(content);
        assert!(api_groups.is_empty());
    }

    #[test]
    fn test_parse_content_without_scopes() {
        let content = r#"\
API One, v1
API Two, v2
"#;
        let api_groups = parse_info_file(content);
        assert_eq!(api_groups.len(), 2);
        assert!(api_groups[0].scopes.is_empty());
        assert!(api_groups[1].scopes.is_empty());
    }

    #[test]
    fn test_parse_content_with_only_header_and_scopes_line() {
        let content = r#"\
API One, v1
Scopes
"#;
        let api_groups = parse_info_file(content);
        assert_eq!(api_groups.len(), 1);
        assert!(api_groups[0].scopes.is_empty());
    }
}