pub use crate::complex_number::ComplexNumbers;
pub use crate::student_score::{Score, StudentInformation};
//programs used to test all cases of:
//complex_number
//
//student_score
#[test]
//testing of addition operation
fn complex_number_addition() {
    let complex_no_sample1 = ComplexNumbers {
        real_number1: 5,
        real_number2: 7,
        imaginary_number1: 12,
        imaginary_number2: 25,
    };
    let output_real = complex_no_sample1.addition();
    assert_eq!(output_real, "12+i37");
}

//testing of subtraction operation
#[test]
fn complex_number_subtraction() {
    let complex_no_sample1 = ComplexNumbers {
        real_number1: 5,
        real_number2: 7,
        imaginary_number1: 12,
        imaginary_number2: 25,
    };
    let output_real = complex_no_sample1.subtraction();
    assert_eq!(output_real, "12+i37");
}

//testing of multiply operation
#[test]
fn complex_number_multiply() {
    let complex_no_sample1 = ComplexNumbers {
        real_number1: 5,
        real_number2: 7,
        imaginary_number1: 12,
        imaginary_number2: 25,
    };
    let output_real = complex_no_sample1.multiplication();
    assert_eq!(output_real, "-265+i209");
}

//testing new details
#[test]
fn new_student() {
    let student_data = Score {
        hindi: 75,
        english: 68,
        maths: 88,
        science: 65,
    };
    let student_1 = StudentInformation::new(&student_data);
    assert_eq!(student_1.name, "Nitin");
    assert_eq!(student_1.roll_no, 25);
    assert_eq!(student_1.department, "Information Technology");
    assert_eq!(student_1.school, "Saint School");
    assert_eq!(student_1.score_of_each_subject.hindi, 75);
    assert_eq!(student_1.score_of_each_subject.english, 68);
    assert_eq!(student_1.score_of_each_subject.maths, 88);
    assert_eq!(student_1.score_of_each_subject.science, 65);
}

//function testing average
//calculating average of 4 subjects
//hindi
//english
//math
//science
#[test]
fn get_average() {
    let student_marks = Score {
        hindi: 43,
        english: 50,
        maths: 52,
        science: 65,
    };
    let check_test = StudentInformation::new(&student_marks);
    assert_eq!(check_test.get_average(), 52 as f32);
}

//function testing pass student
//
//student will pass if he get marks
//equal to 35 or more than 35
#[test]
fn pass_student() {
    let student1_marks = Score {
        hindi: 32,
        english: 31,
        maths: 74,
        science: 55,
    };
    let mut pass_check_test = StudentInformation::new(&student1_marks);
    pass_check_test.student_passed();
    assert_eq!(pass_check_test.score_of_each_subject.hindi, 35);
    assert_eq!(pass_check_test.score_of_each_subject.english, 35);
    assert_eq!(pass_check_test.score_of_each_subject.maths, 74);
    assert_eq!(pass_check_test.score_of_each_subject.science, 55);
}
