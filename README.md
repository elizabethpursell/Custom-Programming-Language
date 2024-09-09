# CSE262 - Programming Languages - Fall 2023

# Final Exam

⏰ **Due by: 12/18/2023 EOD**

## Ethics Contract

**FIRST**: Please read the following carefully:

- I am the sole author of the content within this exam unless otherwise cited.
- I am not an uncredited author of any content in any other exam.
- I will not dicsuss the exam until after the submission deadline.
- All resources I used (including text books and online references, websites, language models), are cited in this exam.
- I will not plagiarize someone else's work and turn it in as my own. If I use someone else's work in this exam, I will cite that work. Failure to cite work I used is plagiarism.
- I understand that acts of academic dishonesty may be penalized to the full extent allowed by the [Lehigh University Code of Conduct][0], including receiving a failing grade for the course. I recognize that I am responsible for understanding the provisions of the Lehigh University Code of Conduct as they relate to this academic exercise.


If you agree with the above, type your full name next to the pen emoji, along with the date. Your exam **will not be graded** without this assent.

---------------------------------------------
🖋️ Elizabeth Pursell 12/13/2023
---------------------------------------------

⚠️ **IMPORTANT:** When you are done, make your first commit with the commit message: `I, <your full name here>, agree to the ethics contract`.

⚠️ **IMPORTANT: As you are working on your midterm, commit your progress regularly.**

## Exam

In this final exam, you are going to design and implement a new feature for the Asa programming language we've been implementing all semester. You must use your Homework 5 solution as the basis for your exam. If you didn't fully complete Homework 5, you must first complete it using the [posted solutions](http://gitlab.cse.lehigh.edu/cse262-programming-languages/fall-2023/assignments/homework-5/-/tree/solutions). Copy and paste the solutions into your repository, and go through the commented lines of code to understand the implementation of the interpreter.

You can choose between three different features to implement, each with a different maximum possible score:

1. comparison operators (B+ 87%)
2. if expressions (A 93%)
3. Cut 1 & Cut 2 (A+ 100%)

You only have to do one of these. No matter which one you choose, you will have to write the grammar, implement the associated parser combinators, modify the interpreter to execute the new feature, and finally write tests to validate its functionality.

## Feature Semantics

### Conditional Operators

If you choose this version of the exam, you'll have to implement the following conditional operators:

1) Greater-than (`>`)
2) Less-than (`<`)
3) Greater-than or Equal-to (`>=`)
4) Less-than or Equal-to (`<=`)
5) Equal-to (`==`)
6) Not Equal-to (`!=`)

A conditional expressions is a number, boolean, identifier or expression; followed by one of the noted operators below; followed by another number, boolean, identifier or expression. Here are some examples of valid and invalid conditional expressions.

Valid conditional expressions:

```
1 > 2
true == x
x > y + z
```
Invalid conditional expressions:

```
1 > true    // invalid beause you can't compare number and boolean
5 - false   // invalid because you can't do math on number and boolean
```

Here's an example demonstrating operator precedence, which you must take into account in your implementation:

```
let x = 10;
let y = 5;
let z = 3;
let result = x + y * z > x * y - z == true;
```

The expression `x + y * z > x * y - z == true` is evaluates as follows:

- `x + y * z` evaluates to `25`
- `x * y - z` evaluates to `47`
- `25 > 47` evaluates to `false`
- `false == true` evaluates to `false`

Use this to help you implement the execution order of comparison operators with respect to the already implemented mathematical operators. Implement as many of these operators as you can for proportional credit.

### if-Expressions

If expressions have the following semantic parts:

- The test expression
- The pass expression
- The fail expression

The syntax of the if expression is the keyword "if" followed by an expression that must evaluate to a boolean value; a list of statements enclosed in curly braces follows; then the keyword "else"; then a second list of statements enclosed in curly braces. Here are some examples of valid and invalid if-expressions.

Vaid if-expressions:

```
// One line form
if true {return false;} else {return true;}
// Multi line form
if true {
    return false;
} else {
    return true;
}
// Else-if is supported
if true {return 1;} else if false {return 2;} else {return 3}
// Result can be assigned to variable
let x = if true {return false;} else {return true;}
```

Invalid if-expressions:

```
// No return for true branch
if true {let x = 1;} else {return true;}
// Missing curly braces
if true {return false;} return true;}
// Inconsistent return types
if true {return 1;} else {return true;}
```
Note: If you choose Cut 2 of the exam, you don't have to parse or interpret conditional operators. You do for Cut 3.

Take the following semantic considerations into account when augmenting the interpreter to support if-expressions:

- Evaluation order: Condition is evaluated first, followed by the true/false branch.
- Type consistency: Ensure both expressions have compatible types.
- Short-circuit evaluation: If condition is true, false branch is not evaluated.
- Return value: The if-expression should return a single value that can be assigned to a variable or used in an expression.

## Part 1 - Grammar

In this part, you must:

- Define the syntax of conditional expressions or if-expressions using EBNF. 
- Define the meaning of the symbols you use to express your grammar. 
- Provide three more examples each of valid and invalid expressions.

## Part 2 - Parser

In this part, you must:

1. Modify the existing parser to support the new grammar rules you defined in Part 1. This will involve adding new parser node variants.
2. Implement the necessary parser combinators for your chosen feature (conditional operators or if-expressions). 

## Part 3 - Interpreter

In this part, you must:

1. Modify the existing interpreter to support the new parser nodes generated by your parser. 
2. Implement the necessary evaluation rules for your chosen feature (conditional operators or if-expressions). 
3. Write at least three tests that demonstrates how your interpreter correctly evaluates your chosen feature. 
4. Ensure your interpreter returns informative error messages for invalid expressions or runtime errors.

## Part 4 - Code Demo and Explanation

This is the oral portion of the exam. You will record an explanation for your interpreter and parser improvements which demonstrates their implementation and functionality. You don't have to show your face but you do have to record your voice (accommodations are available upon request). You should be sure to cover the following points in your discussion:

- Purpose and functionality: Explain what your code does and how it works.

- Grammar: Describe the EBNF grammar you created for your chosen feature and how it defines the syntax of conditional expressions or if-expressions.

- Parser: Explain how your code is organized and structured, discuss any design decisions you made. You should also discuss your coding style and any coding conventions you followed. Demonstrate that the parser matches the grammar by showing what it accepts and rejects.

- Interpreter: Explain how your interpreter has been modified to support the new feature, and discuss the evaluation rules you implemented. Show examples of your interpreter evaluating the chosen feature correctly, and explain how it handles invalid expressions or runtime errors.

If you didn't finish the exam in is entirety, explain how you attempted to solve it and where you got stuck. This will get you at least some points. 

You can use Zoom to do this, [here is a link](https://support.zoom.us/hc/en-us/articles/360059781332-Getting-started-with-recording) to some instructions. You don't have to record your face, only your voice and the screen. Go through the answer and explain how you arrived there. Your goal with this question is to convince me you know what you are talking about, so I want you to do this without reading a script or written answer. Just go through line by line and explain what the program does. When you are done, upload your recording to your Lehigh Drive and add a link below. 

**⚠️IMPORTANT: Make sure you give blanket permission to the link holder to view the file**

🎥 Paste Recording Link(s) Here: https://drive.google.com/file/d/18IbVB7A8N4xmSShyWzNl03I2QtLZti5S/view?usp=sharing

## Submission

Please submit your completed exam, which should include:

1. A detailed description of the feature you chose to implement (conditional operators or if-expressions).
2. The updated EBNF grammar for your chosen feature.
3. The updated parser and interpreter code, including any necessary modifications to support your chosen feature.
4. Test cases demonstrating the correct parsing and evaluation of your chosen feature.
5. A recording link with permission to view granted to the link holder.

- Only files under version control in your forked assignment repository will be graded. Local files left untracked on your computer will not be considered.

- Only code committed *and pushed* prior to the time of grading will be accepted. Locally committed but unpushed code will not be considered.

- Your assignment will be graded according to the [Programming Assignment Grading Rubric](https://drive.google.com/open?id=1V0nBt3Rz6uFMZ9mIaFioLF-48DFX0VdkbgRUDM_eIFk).

Your submission should be organized, well-commented, and easy to understand. Remember to document any assumptions you made during the implementation process, as well as any limitations of your solution. Your final exam will be graded on the correctness, completeness, and clarity of your submission.

## Works Cited

List all sources used during the exam here.
- Used homework-5 solutions as the base for this program
- Some grammar, parser, and interpreter code is from my class-28 lecture notes
- Based grammar structure on my homework-4 submission
- Nom Docs: https://docs.rs/nom/5.0.1/nom/
