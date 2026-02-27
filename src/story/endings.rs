use crate::i18n::LocalizedString;
use crate::story::EndingType;

/// Get the localized title for an ending
pub fn ending_title(ending: &EndingType) -> LocalizedString {
    match ending {
        EndingType::NewDawn => LocalizedString::new("New Dawn", "Aube Nouvelle"),
        EndingType::TheSignal => LocalizedString::new("The Signal", "Le Signal"),
        EndingType::Static => LocalizedString::new("Static", "Gr\u{00e9}sillement"),
        EndingType::GoneDark => LocalizedString::new("Gone Dark", "Signal Perdu"),
        EndingType::TheEsharaWins => LocalizedString::new("The Eshara Wins", "L'Eshara Triomphe"),
    }
}

/// Get the localized description for an ending
pub fn ending_description(ending: &EndingType) -> LocalizedString {
    match ending {
        EndingType::NewDawn => LocalizedString::new(
            "Elara found the safe zone. Your guidance saved her. There is hope.",
            "Elara a trouv\u{00e9} la zone s\u{00fb}re. Tes conseils l'ont sauv\u{00e9}e. Il y a de l'espoir.",
        ),
        EndingType::TheSignal => LocalizedString::new(
            "Elara managed to reverse the Eshara, but at great personal cost.",
            "Elara a r\u{00e9}ussi \u{00e0} inverser l'Eshara, mais \u{00e0} un prix terrible.",
        ),
        EndingType::Static => LocalizedString::new(
            "Elara survived, but the radio broke. Her last words: \"Thank you. For everything. I'll be okay. I think.\"",
            "Elara a surv\u{00e9}cu, mais la radio a l\u{00e2}ch\u{00e9}. Ses derniers mots : \u{00ab} Merci. Pour tout. \u{00c7}a va aller. Je crois. \u{00bb}",
        ),
        EndingType::GoneDark => LocalizedString::new(
            "Elara didn't make it. The radio went silent.",
            "Elara n'a pas surv\u{00e9}cu. La radio s'est tue.",
        ),
        EndingType::TheEsharaWins => LocalizedString::new(
            "Elara was consumed by the phenomena. Her last messages became... wrong.",
            "Elara a \u{00e9}t\u{00e9} absorb\u{00e9}e par les ph\u{00e9}nom\u{00e8}nes. Ses derniers messages sont devenus... \u{00e9}tranges.",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;

    #[test]
    fn test_all_endings_have_titles() {
        let endings = vec![
            EndingType::NewDawn,
            EndingType::TheSignal,
            EndingType::Static,
            EndingType::GoneDark,
            EndingType::TheEsharaWins,
        ];
        for ending in endings {
            let title = ending_title(&ending);
            assert!(!title.get(Language::En).is_empty());
            assert!(!title.get(Language::Fr).is_empty());
            let desc = ending_description(&ending);
            assert!(!desc.get(Language::En).is_empty());
            assert!(!desc.get(Language::Fr).is_empty());
        }
    }
}
