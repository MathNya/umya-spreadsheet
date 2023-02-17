// a:prstGeom
use super::adjust_value_list::AdjustValueList;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct PresetGeometry {
    geometry: String,
    adjust_value_list: AdjustValueList,
}
impl PresetGeometry {
    // Geometryes
    pub const GEOMETRY_ACCENTBORDERCALLOUT1: &'static str = "accentBorderCallout1";
    pub const GEOMETRY_ACCENTBORDERCALLOUT2: &'static str = "accentBorderCallout2";
    pub const GEOMETRY_ACCENTBORDERCALLOUT3: &'static str = "accentBorderCallout3";
    pub const GEOMETRY_ACCENTCALLOUT1: &'static str = "accentCallout1";
    pub const GEOMETRY_ACCENTCALLOUT2: &'static str = "accentCallout2";
    pub const GEOMETRY_ACCENTCALLOUT3: &'static str = "accentCallout3";
    pub const GEOMETRY_ACTIONBUTTONBACKPREVIOUS: &'static str = "actionButtonBackPrevious";
    pub const GEOMETRY_ACTIONBUTTONBEGINNING: &'static str = "actionButtonBeginning";
    pub const GEOMETRY_ACTIONBUTTONBLANK: &'static str = "actionButtonBlank";
    pub const GEOMETRY_ACTIONBUTTONDOCUMENT: &'static str = "actionButtonDocument";
    pub const GEOMETRY_ACTIONBUTTONEND: &'static str = "actionButtonEnd";
    pub const GEOMETRY_ACTIONBUTTONFORWARDNEXT: &'static str = "actionButtonForwardNext";
    pub const GEOMETRY_ACTIONBUTTONHELP: &'static str = "actionButtonHelp";
    pub const GEOMETRY_ACTIONBUTTONHOME: &'static str = "actionButtonHome";
    pub const GEOMETRY_ACTIONBUTTONINFORMATION: &'static str = "actionButtonInformation";
    pub const GEOMETRY_ACTIONBUTTONMOVIE: &'static str = "actionButtonMovie";
    pub const GEOMETRY_ACTIONBUTTONRETURN: &'static str = "actionButtonReturn";
    pub const GEOMETRY_ACTIONBUTTONSOUND: &'static str = "actionButtonSound";
    pub const GEOMETRY_ARC: &'static str = "arc";
    pub const GEOMETRY_BENTARROW: &'static str = "bentArrow";
    pub const GEOMETRY_BENTCONNECTOR2: &'static str = "bentConnector2";
    pub const GEOMETRY_BENTCONNECTOR3: &'static str = "bentConnector3";
    pub const GEOMETRY_BENTCONNECTOR4: &'static str = "bentConnector4";
    pub const GEOMETRY_BENTCONNECTOR5: &'static str = "bentConnector5";
    pub const GEOMETRY_BENTUPARROW: &'static str = "bentUpArrow";
    pub const GEOMETRY_BEVEL: &'static str = "bevel";
    pub const GEOMETRY_BLOCKARC: &'static str = "blockArc";
    pub const GEOMETRY_BORDERCALLOUT1: &'static str = "borderCallout1";
    pub const GEOMETRY_BORDERCALLOUT2: &'static str = "borderCallout2";
    pub const GEOMETRY_BORDERCALLOUT3: &'static str = "borderCallout3";
    pub const GEOMETRY_BRACEPAIR: &'static str = "bracePair";
    pub const GEOMETRY_BRACKETPAIR: &'static str = "bracketPair";
    pub const GEOMETRY_CALLOUT1: &'static str = "callout1";
    pub const GEOMETRY_CALLOUT2: &'static str = "callout2";
    pub const GEOMETRY_CALLOUT3: &'static str = "callout3";
    pub const GEOMETRY_CAN: &'static str = "can";
    pub const GEOMETRY_CHARTPLUS: &'static str = "chartPlus";
    pub const GEOMETRY_CHARTSTAR: &'static str = "chartStar";
    pub const GEOMETRY_CHARTX: &'static str = "chartX";
    pub const GEOMETRY_CHEVRON: &'static str = "chevron";
    pub const GEOMETRY_CHORD: &'static str = "chord";
    pub const GEOMETRY_CIRCULARARROW: &'static str = "circularArrow";
    pub const GEOMETRY_CLOUD: &'static str = "cloud";
    pub const GEOMETRY_CLOUDCALLOUT: &'static str = "cloudCallout";
    pub const GEOMETRY_CORNER: &'static str = "corner";
    pub const GEOMETRY_CORNERTABS: &'static str = "cornerTabs";
    pub const GEOMETRY_CUBE: &'static str = "cube";
    pub const GEOMETRY_CURVEDCONNECTOR2: &'static str = "curvedConnector2";
    pub const GEOMETRY_CURVEDCONNECTOR3: &'static str = "curvedConnector3";
    pub const GEOMETRY_CURVEDCONNECTOR4: &'static str = "curvedConnector4";
    pub const GEOMETRY_CURVEDCONNECTOR5: &'static str = "curvedConnector5";
    pub const GEOMETRY_CURVEDDOWNARROW: &'static str = "curvedDownArrow";
    pub const GEOMETRY_CURVEDLEFTARROW: &'static str = "curvedLeftArrow";
    pub const GEOMETRY_CURVEDRIGHTARROW: &'static str = "curvedRightArrow";
    pub const GEOMETRY_CURVEDUPARROW: &'static str = "curvedUpArrow";
    pub const GEOMETRY_DECAGON: &'static str = "decagon";
    pub const GEOMETRY_DIAGSTRIPE: &'static str = "diagStripe";
    pub const GEOMETRY_DIAMOND: &'static str = "diamond";
    pub const GEOMETRY_DODECAGON: &'static str = "dodecagon";
    pub const GEOMETRY_DONUT: &'static str = "donut";
    pub const GEOMETRY_DOUBLEWAVE: &'static str = "doubleWave";
    pub const GEOMETRY_DOWNARROW: &'static str = "downArrow";
    pub const GEOMETRY_DOWNARROWCALLOUT: &'static str = "downArrowCallout";
    pub const GEOMETRY_ELLIPSE: &'static str = "ellipse";
    pub const GEOMETRY_ELLIPSERIBBON: &'static str = "ellipseRibbon";
    pub const GEOMETRY_ELLIPSERIBBON2: &'static str = "ellipseRibbon2";
    pub const GEOMETRY_FLOWCHARTALTERNATEPROCESS: &'static str = "flowChartAlternateProcess";
    pub const GEOMETRY_FLOWCHARTCOLLATE: &'static str = "flowChartCollate";
    pub const GEOMETRY_FLOWCHARTCONNECTOR: &'static str = "flowChartConnector";
    pub const GEOMETRY_FLOWCHARTDECISION: &'static str = "flowChartDecision";
    pub const GEOMETRY_FLOWCHARTDELAY: &'static str = "flowChartDelay";
    pub const GEOMETRY_FLOWCHARTDISPLAY: &'static str = "flowChartDisplay";
    pub const GEOMETRY_FLOWCHARTDOCUMENT: &'static str = "flowChartDocument";
    pub const GEOMETRY_FLOWCHARTEXTRACT: &'static str = "flowChartExtract";
    pub const GEOMETRY_FLOWCHARTINPUTOUTPUT: &'static str = "flowChartInputOutput";
    pub const GEOMETRY_FLOWCHARTINTERNALSTORAGE: &'static str = "flowChartInternalStorage";
    pub const GEOMETRY_FLOWCHARTMAGNETICDISK: &'static str = "flowChartMagneticDisk";
    pub const GEOMETRY_FLOWCHARTMAGNETICDRUM: &'static str = "flowChartMagneticDrum";
    pub const GEOMETRY_FLOWCHARTMAGNETICTAPE: &'static str = "flowChartMagneticTape";
    pub const GEOMETRY_FLOWCHARTMANUALINPUT: &'static str = "flowChartManualInput";
    pub const GEOMETRY_FLOWCHARTMANUALOPERATION: &'static str = "flowChartManualOperation";
    pub const GEOMETRY_FLOWCHARTMERGE: &'static str = "flowChartMerge";
    pub const GEOMETRY_FLOWCHARTMULTIDOCUMENT: &'static str = "flowChartMultidocument";
    pub const GEOMETRY_FLOWCHARTOFFLINESTORAGE: &'static str = "flowChartOfflineStorage";
    pub const GEOMETRY_FLOWCHARTOFFPAGECONNECTOR: &'static str = "flowChartOffpageConnector";
    pub const GEOMETRY_FLOWCHARTONLINESTORAGE: &'static str = "flowChartOnlineStorage";
    pub const GEOMETRY_FLOWCHARTOR: &'static str = "flowChartOr";
    pub const GEOMETRY_FLOWCHARTPREDEFINEDPROCESS: &'static str = "flowChartPredefinedProcess";
    pub const GEOMETRY_FLOWCHARTPREPARATION: &'static str = "flowChartPreparation";
    pub const GEOMETRY_FLOWCHARTPROCESS: &'static str = "flowChartProcess";
    pub const GEOMETRY_FLOWCHARTPUNCHEDCARD: &'static str = "flowChartPunchedCard";
    pub const GEOMETRY_FLOWCHARTPUNCHEDTAPE: &'static str = "flowChartPunchedTape";
    pub const GEOMETRY_FLOWCHARTSORT: &'static str = "flowChartSort";
    pub const GEOMETRY_FLOWCHARTSUMMINGJUNCTION: &'static str = "flowChartSummingJunction";
    pub const GEOMETRY_FLOWCHARTTERMINATOR: &'static str = "flowChartTerminator";
    pub const GEOMETRY_FOLDERCORNER: &'static str = "folderCorner";
    pub const GEOMETRY_FRAME: &'static str = "frame";
    pub const GEOMETRY_FUNNEL: &'static str = "funnel";
    pub const GEOMETRY_GEAR6: &'static str = "gear6";
    pub const GEOMETRY_GEAR9: &'static str = "gear9";
    pub const GEOMETRY_HALFFRAME: &'static str = "halfFrame";
    pub const GEOMETRY_HEART: &'static str = "heart";
    pub const GEOMETRY_HEPTAGON: &'static str = "heptagon";
    pub const GEOMETRY_HEXAGON: &'static str = "hexagon";
    pub const GEOMETRY_HOMEPLATE: &'static str = "homePlate";
    pub const GEOMETRY_HORIZONTALSCROLL: &'static str = "horizontalScroll";
    pub const GEOMETRY_IRREGULARSEAL1: &'static str = "irregularSeal1";
    pub const GEOMETRY_IRREGULARSEAL2: &'static str = "irregularSeal2";
    pub const GEOMETRY_LEFTARROW: &'static str = "leftArrow";
    pub const GEOMETRY_LEFTARROWCALLOUT: &'static str = "leftArrowCallout";
    pub const GEOMETRY_LEFTBRACE: &'static str = "leftBrace";
    pub const GEOMETRY_LEFTBRACKET: &'static str = "leftBracket";
    pub const GEOMETRY_LEFTCIRCULARARROW: &'static str = "leftCircularArrow";
    pub const GEOMETRY_LEFTRIGHTARROW: &'static str = "leftRightArrow";
    pub const GEOMETRY_LEFTRIGHTARROWCALLOUT: &'static str = "leftRightArrowCallout";
    pub const GEOMETRY_LEFTRIGHTCIRCULARARROW: &'static str = "leftRightCircularArrow";
    pub const GEOMETRY_LEFTRIGHTRIBBON: &'static str = "leftRightRibbon";
    pub const GEOMETRY_LEFTRIGHTUPARROW: &'static str = "leftRightUpArrow";
    pub const GEOMETRY_LEFTUPARROW: &'static str = "leftUpArrow";
    pub const GEOMETRY_LIGHTNINGBOLT: &'static str = "lightningBolt";
    pub const GEOMETRY_LINE: &'static str = "line";
    pub const GEOMETRY_LINEINV: &'static str = "lineInv";
    pub const GEOMETRY_MATHDIVIDE: &'static str = "mathDivide";
    pub const GEOMETRY_MATHEQUAL: &'static str = "mathEqual";
    pub const GEOMETRY_MATHMINUS: &'static str = "mathMinus";
    pub const GEOMETRY_MATHMULTIPLY: &'static str = "mathMultiply";
    pub const GEOMETRY_MATHNOTEQUAL: &'static str = "mathNotEqual";
    pub const GEOMETRY_MATHPLUS: &'static str = "mathPlus";
    pub const GEOMETRY_MOON: &'static str = "moon";
    pub const GEOMETRY_NONISOSCELESTRAPEZOID: &'static str = "nonIsoscelesTrapezoid";
    pub const GEOMETRY_NOSMOKING: &'static str = "noSmoking";
    pub const GEOMETRY_NOTCHEDRIGHTARROW: &'static str = "notchedRightArrow";
    pub const GEOMETRY_OCTAGON: &'static str = "octagon";
    pub const GEOMETRY_PARALLELOGRAM: &'static str = "parallelogram";
    pub const GEOMETRY_PENTAGON: &'static str = "pentagon";
    pub const GEOMETRY_PIE: &'static str = "pie";
    pub const GEOMETRY_PIEWEDGE: &'static str = "pieWedge";
    pub const GEOMETRY_PLAQUE: &'static str = "plaque";
    pub const GEOMETRY_PLAQUETABS: &'static str = "plaqueTabs";
    pub const GEOMETRY_PLUS: &'static str = "plus";
    pub const GEOMETRY_QUADARROW: &'static str = "quadArrow";
    pub const GEOMETRY_QUADARROWCALLOUT: &'static str = "quadArrowCallout";
    pub const GEOMETRY_RECT: &'static str = "rect";
    pub const GEOMETRY_RIBBON: &'static str = "ribbon";
    pub const GEOMETRY_RIBBON2: &'static str = "ribbon2";
    pub const GEOMETRY_RIGHTARROW: &'static str = "rightArrow";
    pub const GEOMETRY_RIGHTARROWCALLOUT: &'static str = "rightArrowCallout";
    pub const GEOMETRY_RIGHTBRACE: &'static str = "rightBrace";
    pub const GEOMETRY_RIGHTBRACKET: &'static str = "rightBracket";
    pub const GEOMETRY_ROUND1RECT: &'static str = "round1Rect";
    pub const GEOMETRY_ROUND2DIAGRECT: &'static str = "round2DiagRect";
    pub const GEOMETRY_ROUND2SAMERECT: &'static str = "round2SameRect";
    pub const GEOMETRY_ROUNDRECT: &'static str = "roundRect";
    pub const GEOMETRY_RTTRIANGLE: &'static str = "rtTriangle";
    pub const GEOMETRY_SMILEYFACE: &'static str = "smileyFace";
    pub const GEOMETRY_SNIP1RECT: &'static str = "snip1Rect";
    pub const GEOMETRY_SNIP2DIAGRECT: &'static str = "snip2DiagRect";
    pub const GEOMETRY_SNIP2SAMERECT: &'static str = "snip2SameRect";
    pub const GEOMETRY_SNIPROUNDRECT: &'static str = "snipRoundRect";
    pub const GEOMETRY_SQUARETABS: &'static str = "squareTabs";
    pub const GEOMETRY_STAR10: &'static str = "star10";
    pub const GEOMETRY_STAR12: &'static str = "star12";
    pub const GEOMETRY_STAR16: &'static str = "star16";
    pub const GEOMETRY_STAR24: &'static str = "star24";
    pub const GEOMETRY_STAR32: &'static str = "star32";
    pub const GEOMETRY_STAR4: &'static str = "star4";
    pub const GEOMETRY_STAR5: &'static str = "star5";
    pub const GEOMETRY_STAR6: &'static str = "star6";
    pub const GEOMETRY_STAR7: &'static str = "star7";
    pub const GEOMETRY_STAR8: &'static str = "star8";
    pub const GEOMETRY_STRAIGHTCONNECTOR1: &'static str = "straightConnector1";
    pub const GEOMETRY_STRIPEDRIGHTARROW: &'static str = "stripedRightArrow";
    pub const GEOMETRY_SUN: &'static str = "sun";
    pub const GEOMETRY_SWOOSHARROW: &'static str = "swooshArrow";
    pub const GEOMETRY_TEARDROP: &'static str = "teardrop";
    pub const GEOMETRY_TRAPEZOID: &'static str = "trapezoid";
    pub const GEOMETRY_TRIANGLE: &'static str = "triangle";
    pub const GEOMETRY_UPARROW: &'static str = "upArrow";
    pub const GEOMETRY_UPARROWCALLOUT: &'static str = "upArrowCallout";
    pub const GEOMETRY_UPDOWNARROW: &'static str = "upDownArrow";
    pub const GEOMETRY_UPDOWNARROWCALLOUT: &'static str = "upDownArrowCallout";
    pub const GEOMETRY_UTURNARROW: &'static str = "uturnArrow";
    pub const GEOMETRY_VERTICALSCROLL: &'static str = "verticalScroll";
    pub const GEOMETRY_WAVE: &'static str = "wave";
    pub const GEOMETRY_WEDGEELLIPSECALLOUT: &'static str = "wedgeEllipseCallout";
    pub const GEOMETRY_WEDGERECTCALLOUT: &'static str = "wedgeRectCallout";
    pub const GEOMETRY_WEDGEROUNDRECTCALLOUT: &'static str = "wedgeRoundRectCallout";

    pub fn get_geometry(&self) -> &str {
        &self.geometry
    }

    pub fn set_geometry<S: Into<String>>(&mut self, value: S) {
        self.geometry = value.into();
    }

    pub fn get_adjust_value_list(&self) -> &AdjustValueList {
        &self.adjust_value_list
    }

    pub fn get_adjust_value_list_mut(&mut self) -> &mut AdjustValueList {
        &mut self.adjust_value_list
    }

    pub fn set_adjust_value_list(&mut self, value: AdjustValueList) {
        self.adjust_value_list = value;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.set_geometry(get_attribute(e, b"prst").unwrap());

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:avLst" => {
                        self.get_adjust_value_list_mut().set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:prstGeom" => {
                        return;
                    }
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:prstGeom"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:prstGeom
        write_start_tag(writer, "a:prstGeom", vec![("prst", &self.geometry)], false);

        // a:avLst
        let _ = &self.adjust_value_list.write_to(writer);

        write_end_tag(writer, "a:prstGeom");
    }
}
