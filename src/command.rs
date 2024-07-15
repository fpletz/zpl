#[derive(Clone)]
pub enum PostPrintAction {
    /// Present only, let user tear off.
    TearOff,
    /// Present and cut.
    Cut,
}

#[derive(Clone)]
pub enum MediaType {
    /// Color is in the label, turning dark on heating.
    Direct,
    /// Color in a separate color strip.
    Transfer,
}

#[derive(Clone)]
pub enum ZplCommand {
    Raw {
        text: String,
        /// How many 'lines' / fields of text to read back after the sequence, to purge what the
        /// print would send from the buffer. If we want to raw-ify commands that send non-text
        /// delimited fields then we should look into having a proper sequence encoded in this
        /// field.
        response_lines: u32,
    },
    Magic,
    PersistConfig,
    SetDarkness(usize),
    SetEncoding(usize),
    SetInverted(bool),
    SetHalfDensity(bool),
    SetHome(u32, u32),
    SetSpeed {
        print: usize,
        slew: usize,
    },
    SetMediaType(MediaType),
    LabelSetup {
        w: u32,
        h: u32,
        dots: u32,
    },
    SetPostPrintAction(PostPrintAction),
    SetHorizontalShift(usize),
    SetVerticalShift(isize),
    SetTearOffPosition(isize),
    MoveOrigin(u32, u32),
    PrintQuantity {
        total: u32,
        pause_and_cut_after: u32,
        replicates: u32,
        cut_only: bool,
    },
    HostIndication,
    HostRamStatus,
    HostStatusReturn,
    Start,
    End,
}

impl ZplCommand {
    /// How many lines of text
    pub fn how_many_lines_of_text(&self) -> u32 {
        match self {
            ZplCommand::HostIndication => 1,
            ZplCommand::HostRamStatus => 1,
            ZplCommand::HostStatusReturn => 3,
            _ => 0,
        }
    }
}

impl From<ZplCommand> for String {
    fn from(value: ZplCommand) -> Self {
        match value {
            ZplCommand::Raw { text, .. } => text,
            ZplCommand::Magic => vec!["CT~~CD,~CC^~CT~", "^XA~TA000~JSN^LT0^MNW"].join("\n"),
            // Removed:
            // -
            // - ^PON -> rotate by 180 degrees
            // - ^PMN -> decommission?
            ZplCommand::PersistConfig => "^JUS".to_string(),
            ZplCommand::SetHalfDensity(d) => format!("^JM{}", if d { "B" } else { "A" }),
            ZplCommand::SetDarkness(e) => format!("~SD{}", e),
            ZplCommand::SetEncoding(e) => format!("^CI{}", e),
            ZplCommand::SetHome(x, y) => format!("^LH{},{}", x, y),
            ZplCommand::SetInverted(i) => {
                format!("^LR{}", if i { "Y" } else { "N" })
            }
            ZplCommand::SetMediaType(t) => {
                let t = match t {
                    MediaType::Direct => "D",
                    MediaType::Transfer => "T",
                };
                format!("^MT{}", t)
            }
            ZplCommand::SetSpeed { print, slew } => format!("^PR{},{}", print, slew),
            ZplCommand::LabelSetup { w, h, dots } => {
                format!("^PW{:0>3}\n^LL{:0>4}", w * dots, h * dots)
            }
            ZplCommand::SetPostPrintAction(a) => {
                let c = match a {
                    PostPrintAction::TearOff => "T",
                    PostPrintAction::Cut => "C",
                };

                format!("^MM{}", c)
            }
            ZplCommand::SetHorizontalShift(s) => format!("^LS{}", s),
            ZplCommand::SetVerticalShift(s) => format!("^LT{}", s),
            ZplCommand::SetTearOffPosition(p) => format!("~TA{:>+04}", p),
            ZplCommand::MoveOrigin(x, y) => format!("^FO{},{}", x, y),
            ZplCommand::PrintQuantity {
                total,
                pause_and_cut_after,
                replicates,
                cut_only,
            } => {
                format!(
                    "^PQ{},{},{},{}",
                    total,
                    pause_and_cut_after,
                    replicates,
                    if cut_only { "Y" } else { "N" }
                )
            }
            ZplCommand::Start => "^XA".to_string(),
            ZplCommand::End => "^XZ".to_string(),
            ZplCommand::HostIndication => "~HI".to_string(),
            ZplCommand::HostRamStatus => "~HM".to_string(),
            ZplCommand::HostStatusReturn => "~HS".to_string(),
        }
    }
}

#[test]
fn test_raw() {
    let c = ZplCommand::Raw {
        text: "Abc".to_string(),
        response_lines: 0,
    };

    assert_eq!(String::from(c), "Abc");
}

#[test]
fn test_setup() {
    let c = ZplCommand::LabelSetup {
        w: 57,
        h: 32,
        dots: 12,
    };
    assert_eq!(String::from(c), "^PW684\n^LL0384");
}
