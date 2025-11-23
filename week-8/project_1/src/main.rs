use std::io;

fn aps_level(role: &str) -> String{
    let aps_1_2 = vec!["Intern"," ","Paralegal","Placement"];
    let aps_3_5 = vec!["Administrator","Reasearch Assistant","Junior Associate","Classroom Teacher"];
    let aps_5_8 = vec!["Senior Administrator","PhD Candidate","Associate","Senior Teacher"];
    let el1_8_10 = vec!["Office Manager","Post-Doc Researcher","Senior Associate 1-2","Leading Teacher"];
    let el2_10_13 = vec!["Director","Senior Lecturer","Senior Associate 2-3","Deputy Principal"];
    let ses = vec!["CEO","Dean","Partner","Principal"];

    for item in aps_1_2{
        if item == role{
            return "APS 1-2".to_string();
        }
    }
    for item in aps_3_5{
        if item == role{
            return "APS 3-5".to_string();
        }
    }
    for item in aps_5_8{
        if item == role{
            return "APS 5-8".to_string();
        }
    }
    for item in el1_8_10{
        if item == role{
            return "EL1 8-10".to_string();
        }
    }
    for item in el2_10_13{
        if item == role{
            return "EL2 10-13".to_string();
        }
    }
    for item in ses{
        if item == role{
            return "SES".to_string();
        }
    }
    return "Role not found".to_string();
}

fn main(){
    println!("Enter staff role: ");

    let mut role = String::new();
    io::stdin().read_line(&mut role).expect("Invalid");
    let _result:String = role.trim().parse().expect("Invalid");

    let level = aps_level(&role);
    println!("Staff level: {}",level );
}