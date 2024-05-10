// implimentation of Student

pub mod falulty {
    pub enum Qualification {
        BEd,
        BSc,
        BTech,
        BArt,
        MArt,
        MSc,
        MTech,
        Other,
        BBa,
        MBa,
    }

    pub enum Position {
        Principle,
        VPrinnciple,
        Accountance,
        Teacher,
        Clerk,
    }
    pub struct Faculty {
        name: String,
        age: u8,
        f_id: u32,
        qualification: Qualification,
        position: Position,
        can_teach: u8,
    }

    impl Faculty{
        pub fn new(name:String,age: u8, f_id:u32, qualification: Qualification , positon: Position, can_teach:u8) -> Faculty {
            Faculty{
                name: name, 
                age : age, 
                f_id: f_id,
                qualification: qualification,
                position: positon, 
                can_teach: can_teach
            }
        }
        pub fn info(self) {
            println!("{}", self.name);
        }
    }
}
pub mod student {
    pub enum Stream {
        General,
        PCM,
        PCB,
        Both,
    }
    pub struct Student {
        name: String,   // Name of the Student
        age: u8,        // age of the Studne
        standard: u8,   // Standard of the student
        stream: Stream, // Stream of the student only avilable if the standard of the student is >10
        roll_no: u32,   // Roll number of the student must be uniquely identifable.
    }
}

pub mod school {
    pub enum Affiliation {
        CBSE,
        HPBOSE,
    }
    pub struct School {
        name: String,
        s_id: u8,
        affiliation: Affiliation,
        principle: crate::falulty::Faculty,
        // TODO
    }
}
