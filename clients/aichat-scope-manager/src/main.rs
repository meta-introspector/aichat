use std::fs;
use std::path::PathBuf;

use clap::Parser;
use dialoguer::{Input, MultiSelect, Select};
use quote::quote;
use heck::AsPascalCase;

use aichat_scope_manager::{parse_info_file, ApiGroup, Scope};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the info.txt file
    #[arg(short, long, default_value = "vendor/google-apis/tlns-google-oauth2/tlns-google-oauth2-proc/info.txt")]
    info_file: PathBuf,

    /// Name for the generated policy (e.g., "AdminAccess")
    #[arg(short, long)]
    policy_name: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let content = fs::read_to_string(&args.info_file)
        .map_err(|e| anyhow::anyhow!("Failed to read info file: {}", e))?;

    let api_groups = parse_info_file(&content);

    if api_groups.is_empty() {
        println!("No API groups found in the info file.");
        return Ok(())
    }

    // Select API Group
    let api_group_names: Vec<String> = api_groups.iter().map(|g| g.name.clone()).collect();
    let selected_api_group_index = Select::new()
        .with_prompt("Select an API group")
        .items(&api_group_names)
        .interact()?;

    let selected_api_group = &api_groups[selected_api_group_index];

    // Select Scopes within the selected API Group
    let scope_items: Vec<String> = selected_api_group
        .scopes
        .iter()
        .map(|s| format!("{} - {}", s.url, s.description))
        .collect();

    if scope_items.is_empty() {
        println!("No scopes found for the selected API group.");
        return Ok(())
    }

    let chosen_scopes_indices = MultiSelect::new()
        .with_prompt("Select scopes (use space to select/deselect, enter to confirm)")
        .items(&scope_items)
        .interact()?;

    let selected_scope_urls: Vec<String> = chosen_scopes_indices
        .iter()
        .map(|&i| selected_api_group.scopes[i].url.clone())
        .collect();

    println!("\nSelected Scopes:");
    for url in &selected_scope_urls {
        println!("{}", url);
    }

    if let Some(policy_name) = args.policy_name {
        let policy_variant_name = heck::AsPascalCase(&policy_name).to_string();
        let policy_file_path = PathBuf::from("clients/aichat-policies/src/generated_policies.rs");

        let scope_literals = selected_scope_urls.iter().map(|url| quote! { #url }).collect::<Vec<_>>();

        let generated_code = quote! {
            use tlns_google_oauth2::scopes::Scopes;
            use tlns_google_oauth2::ToGoogleScope;

            pub enum Policy {
                #policy_variant_name,
                // Policy variants will be generated here by aichat-scope-manager
            }

            impl Policy {
                pub fn required_scopes(&self) -> Vec<&'static str> {
                    match self {
                        Policy::#policy_variant_name => vec![ 
                            #(#scope_literals),*
                        ],
                        // Match arms for required scopes will be generated here
                    }
                }

                pub fn check_access(&self, granted_scopes: &[Scopes]) -> bool {
                    self.required_scopes().iter().all(|req_scope| {
                        granted_scopes.iter().any(|granted_scope| granted_scope.to_google_scope() == *req_scope)
                    })
                }
            }

            #[cfg(test)]
            mod tests {
                use super::Policy;
                use tlns_google_oauth2::scopes::Scopes;

                // Test cases will be added here after policies are generated
                // Example for #policy_variant_name:
                /*
                #[test]
                fn test_#policy_variant_name_policy() {
                    let granted_scopes = vec![ 
                        // Add relevant scopes here
                    ];
                    assert!(Policy::#policy_variant_name.check_access(&granted_scopes));
                }
                */
            }
        };

        fs::write(&policy_file_path, generated_code.to_string())
            .map_err(|e| anyhow::anyhow!("Failed to write generated policy file: {}", e))?;

        println!("\nGenerated policy enum code at {}", policy_file_path.display());
        println!("Please copy the content of this file into clients/aichat-policies/src/lib.rs");
    }

    Ok(())
}
