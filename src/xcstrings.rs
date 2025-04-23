use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

const SUPPORTED_VERSION: &str = "1.0";

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StringUnitState {
    Translated,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StringUnit {
    // pub state: StringUnitState,
    pub value: String,
}

// Represents the localization details for a specific language.
// A localization can either be a single StringUnit or have Variations.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Localization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_unit: Option<StringUnit>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ExtractionState {
    Manual,
}

// Represents a single string entry in the main "strings" map.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StringEntry {
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub comment: Option<String>,
    // pub extraction_state: ExtractionState,
    pub localizations: HashMap<String, Localization>,
}

// Represents the root structure of the .xcstrings file.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    // pub source_language: String,
    pub version: String,
    pub strings: HashMap<String, StringEntry>,
}

impl Root {
    pub fn strings_for_localization(&self, locale: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let strings = &self.strings;

        for (key, entry) in strings.iter() {
            if let Some(localized) = entry.localizations.get(locale) {
                if let Some(unit) = &localized.string_unit {
                    // Insert localized string into map
                    map.insert(key.to_string(), unit.value.to_string());
                }
                // ignore if not of string unit type
            } else {
                eprintln!(
                    "{}: cannot find localized string of {} for locale {}",
                    "warning".yellow().bold(),
                    key,
                    locale
                );
                continue;
            }
        }

        map
    }

    // pub fn locales(&self) -> Vec<String> {
    //     self.strings
    //         .values()
    //         .flat_map(|entry| entry.localizations.keys().cloned())
    //         .collect::<HashSet<_>>() // deduplicate
    //         .into_iter()
    //         .collect()
    // }

    pub fn all_strings(&self) -> HashMap<String, HashMap<String, String>> {
        let mut all_strings_map: HashMap<String, HashMap<String, String>> = HashMap::new();

        for (key, entry) in &self.strings {
            for (locale, unit) in &entry.localizations {
                // Get or insert a new HashMap for the locale if it doesn't exist
                let locale_map = all_strings_map
                    .entry(locale.to_string()) // Get the entry for the locale, or insert a new one if not present
                    .or_default(); // Insert a new HashMap if it didn't exist

                if let Some(unit) = &unit.string_unit {
                    // Insert localized string into map
                    locale_map.insert(key.to_string(), unit.value.to_string());
                }
                // ignore if not of string unit type
            }
        }

        all_strings_map
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to parse xcstrings file")]
    Parse(#[from] serde_json::Error),
    #[error("unsupported version (expected {expected:?}, found {found:?})")]
    UnsupportedVersion { expected: String, found: String },
}

pub fn deserialize(catalog: &str) -> Result<Root, crate::xcstrings::Error> {
    let data = serde_json::from_str::<Root>(catalog)?;

    if data.version != SUPPORTED_VERSION {
        return Err(Error::UnsupportedVersion {
            expected: (SUPPORTED_VERSION.to_string()),
            found: (data.version),
        });
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const XCSTRINGS_JSON: &str = r#"
    {
      "sourceLanguage" : "en",
      "strings" : {
        "%lld books" : {
          "extractionState" : "manual",
          "localizations" : {
            "de" : {
              "variations" : {
                "plural" : {
                  "one" : {
                    "stringUnit" : {
                      "state" : "translated",
                      "value" : "%lld Buch"
                    }
                  },
                  "other" : {
                    "stringUnit" : {
                      "state" : "translated",
                      "value" : "%lld Bücher"
                    }
                  }
                }
              }
            },
            "en" : {
              "variations" : {
                "plural" : {
                  "one" : {
                    "stringUnit" : {
                      "state" : "translated",
                      "value" : "%lld book"
                    }
                  },
                  "other" : {
                    "stringUnit" : {
                      "state" : "translated",
                      "value" : "%lld books"
                    }
                  }
                }
              }
            }
          }
        },
        "device" : {
          "comment" : "this is a comment",
          "extractionState" : "manual",
          "localizations" : {
            "de" : {
              "variations" : {
                "device" : {
                  "mac" : {
                    "stringUnit" : {
                      "state" : "translated",
                      "value" : "Mac"
                    }
                  },
                  "other" : {
                    "stringUnit" : {
                      "state" : "translated",
                      "value" : "Andere Geräte"
                    }
                  }
                }
              }
            },
            "en" : {
              "variations" : {
                "device" : {
                  "mac" : {
                    "stringUnit" : {
                      "state" : "translated",
                      "value" : "Mac"
                    }
                  },
                  "other" : {
                    "stringUnit" : {
                      "state" : "translated",
                      "value" : "Other Devices"
                    }
                  }
                }
              }
            }
          }
        },
        "farewell" : {
          "comment" : "comment 2",
          "extractionState" : "manual",
          "localizations" : {
            "de" : {
              "stringUnit" : {
                "state" : "translated",
                "value" : "auf wiedersehen"
              }
            },
            "en" : {
              "stringUnit" : {
                "state" : "translated",
                "value" : "goodbye"
              }
            }
          }
        },
        "greeting" : {
          "extractionState" : "manual",
          "localizations" : {
            "de" : {
              "stringUnit" : {
                "state" : "translated",
                "value" : "hallo"
              }
            },
            "en" : {
              "stringUnit" : {
                "state" : "translated",
                "value" : "hello"
              }
            }
          }
        }
      },
      "version" : "1.0"
    }
    "#;

    #[test]
    fn test_parse() {
        let result = deserialize(XCSTRINGS_JSON);
        assert!(result.is_ok());

        let root = result.unwrap();
        assert_eq!(root.version, "1.0");
        // assert_eq!(root.source_language, "en");

        assert!(root.strings.contains_key("farewell"));
    }

    #[test]
    fn test_root_fields_parsed_correctly() {
        let parsed = deserialize(XCSTRINGS_JSON).expect("Parsing failed");
        // assert_eq!(parsed.source_language, "en");
        assert_eq!(parsed.version, "1.0");
        assert_eq!(parsed.strings.len(), 4);
    }

    #[test]
    fn test_string_entry_fields() {
        let parsed = deserialize(XCSTRINGS_JSON).expect("Parsing failed");

        // Check the "device" entry specifically
        let device_entry = parsed.strings.get("device").expect("Missing 'device' key");
        // assert_eq!(device_entry.comment, Some("this is a comment".to_string()));
        // assert_eq!(device_entry.extraction_state, ExtractionState::Manual);
        assert!(device_entry.localizations.contains_key("de"));
        assert!(device_entry.localizations.contains_key("en"));

        // Check the "greeting" entry (no comment)
        // let greeting_entry = parsed
        //     .strings
        //     .get("greeting")
        //     .expect("Missing 'greeting' key");
        // assert_eq!(greeting_entry.comment, None);
        // assert_eq!(greeting_entry.extraction_state, ExtractionState::Manual);
    }

    #[test]
    fn test_localization_with_string_unit() {
        let parsed = deserialize(XCSTRINGS_JSON).expect("Parsing failed");
        let farewell_entry = parsed
            .strings
            .get("farewell")
            .expect("Missing 'farewell' key");
        let de_loc = farewell_entry
            .localizations
            .get("de")
            .expect("Missing 'de' localization");

        // Assert that this localization uses string_unit, not variations
        assert!(de_loc.string_unit.is_some());

        let unit = de_loc.string_unit.as_ref().unwrap();
        // assert_eq!(unit.state, StringUnitState::Translated);
        assert_eq!(unit.value, "auf wiedersehen");
    }
}
