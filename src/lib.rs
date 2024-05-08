// implimentation of Student 

enum Stream{
    General, 
    PCM,
    PCB, 
    Both
}

enum Qualification{
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

enum Position{
    Principle, 
    VPrinnciple, 
    Accountance, 
    Teacher,
    Clerk, 
}


enum Affiliation{
    CBSE, 
    HPBOSE
}

struct Student{
    name : String,  // Name of the Student  
    age : u8,  // age of the Studne 
    standard : u8,  // Standard of the student
    stream : Stream, // Stream of the student only avilable if the standard of the student is >10 
    roll_no : u32,  // Roll number of the student must be uniquely identifable.
}


struct Faculty{
    name: String, 
    age : u8, 
    f_id: u32, 
    qualification : Qualification, 
    position : Position, 
    can_teach : u8,
}


struct School{
    name : String, 
    s_id : u8, 
    affiliation : Affiliation, 
    principle : Faculty, 
    // TODO
}