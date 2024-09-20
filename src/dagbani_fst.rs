use crate::fst::FST;
use crate::pw::PhonologicalWord;

pub struct DagbaniFST {
    fst: FST,
}

impl DagbaniFST {
    pub fn new() -> Self {
        let mut fst = FST::new();
        
        fst.add_transition("start", "<", "<", "λ");
        fst.add_transition("<", "C_Onset", "C_Onset", "λ");
        fst.add_transition("C_Onset", "V_Nucleus", "V", "μ");
        fst.add_transition("C_Onset", "C_Onset", "C_Onset", "λ");

        fst.add_transition("V_Nucleus", "C_Onset", "C_Onset", "λ");
        fst.add_transition("V_Nucleus", "V_Nucleus", "V", "μ");
        fst.add_transition("V_Nucleus", "C_Coda", "C_Coda", "μ");
        fst.add_transition("V_Nucleus", ">", ">", "λ");

        fst.add_transition("C_Coda", "C_Coda", "C_Coda", "μ");
        fst.add_transition("C_Coda", "C_Onset", "C_Onset", "λ");
        fst.add_transition("C_Coda", ">", ">", "λ");
        
        DagbaniFST { fst }
    }

    pub fn process(&self, word: &PhonologicalWord) -> Result<String, String> {
        let mut cv_structure = word.get_cv_structure();
        // turn into form <CVC>
        // makes it suitable for FST processing
        let start_symbol = "<";
        let end_symbol = ">";
        cv_structure.insert(0, start_symbol);
        cv_structure.push(end_symbol);
        println!("{}", cv_structure.join(" "));

        self.fst.process(cv_structure)
    }
}