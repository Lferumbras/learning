/*
Challenge Questions:

    Modify the code to create a function that takes a vector and a value as input and inserts the value at the beginning and end of the vector. 
    Invoke this function with the `v` vector and a new value of your choice to add elements at both ends of the vector. Print the updated vector to 
    verify your changes.

    Extend the program by implementing a function that takes two vectors as input and appends the elements of the second vector to the first vector. 
    Invoke this function with the `v` vector and another vector of your choice. Print the updated vector to ensure the elements are correctly appended.

*/

fn insert_values(mut v : Vec<i32>, element : i32){
    v.push(element);
    v.insert(0, element);
    println!("{:?}", v);
}

fn union_vectors(mut v : Vec<i32>, v2 : &[i32]){
    v.extend(v2);
    println!("{:?}", v);
}


fn main() {
    let mut v = vec![1, 2, 3];
    let v2 = vec![4,5,6];

    insert_values(v.clone(), 9);
    union_vectors(v.clone(), &v2);
    //println!("{:?}", v);
    //v.push(4);
    //println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    //let more_numbers = vec![5, 6];
    //v.extend(more_numbers);
    //println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    //let mut other_numbers = vec![7, 8];
    //v.append(&mut other_numbers);
    //println!("{:?}", v);

    // insert items at a given index
    //v.insert(0, 0);
   // println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8] 
}
