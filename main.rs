//name: Serena Theobald

//link to Stanford dataset I'm using: https://snap.stanford.edu/data/ca-AstroPh.html


use std::collections::HashMap;

use std::fs::File;
use std::io::Read;

use std::io::{BufRead, BufReader};

type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;



fn main()
{
    // Create the hashmap to map old ids to new ids
    // data lowest to highest and then for each row, if the old_id hasn't already been added into the map, insert it into the hashmap (the old_id, the new id which will be a counter starting at 0)
    
    //let mut node_id_map = HashMap::new();

    let mut value: usize = 0;

    let filename = "data.txt";

    //Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut raw_edges: ListOfEdges = Vec::new();
    let mut int_arr = vec![];
    let mut n: usize = 0;




    for (index, line) in reader.lines().enumerate()
    {
        let line = line.unwrap(); //ignore errors
        let line_iter = line.split_whitespace();

        if index == 0{

            //trim: removes leading and trailing whitespace
            //parse: to convert line of string type into usize
            //unwrap: ignores errors
            n = line.trim().parse().unwrap();
           
        }
        else if index> 0{

            int_arr = vec![];

            for val in line_iter{
                value = val.parse().unwrap();
                int_arr.push(value);

            }
            raw_edges.push((int_arr[0], int_arr[1]));
        }

    }

    let mut raw_edges_copy = raw_edges;

    //create sorted vector to create hashmap to avoid duplicates
    raw_edges_copy.sort_by(|b, a| b.0.partial_cmp(&a.0).unwrap());
    //println!("{:?}", raw_edges_copy);
    let mut new_node_id = 0;
    let mut last_old_node_id = 0;
    let mut node_id_map =  HashMap::new();
    

    for i in 0..raw_edges_copy.len()
    {

   
        if raw_edges_copy[i].0 == last_old_node_id{

            continue;
        }
        else if raw_edges_copy[i].0 != last_old_node_id
        {
            node_id_map.insert(raw_edges_copy[i].0, new_node_id);
            last_old_node_id = raw_edges_copy[i].0;
            new_node_id = new_node_id +1;
        }

       
    }

    //println!("{}", raw_edges_copy.len());
    //println!("{}", new_node_id);

    for i in 0..raw_edges_copy.len()
    {

        //Updated raw_edges[i].0 and raw_edges[i].1 to be the corresponding hasmap entry
   

      
        match node_id_map.get(&raw_edges_copy[i].0)
        {
            Some(&id) => raw_edges_copy[i].0 = id,
            _ => println!("Did not find the record, which means something went wrong"),
        }
         match node_id_map.get(&raw_edges_copy[i].1)
        {
            Some(&id) => raw_edges_copy[i].1 = id,
            _ => println!("Did not find the record, which means something went wrong"),
        }

    }

    let n = new_node_id;
    //println!("{}", n);

    println!();



    let mut graph_list: Vec<Vec<usize>> = vec![vec![];n];
    for (v,w) in raw_edges_copy.iter(){
        graph_list[*v].push(*w); 
      

    };

    let mut num_degrees = 0;
    let mut num_nodes = 0;
    let mut degree_distribution: f32 = 0.0;
    
    let mut num_degree_array = vec![];
    let mut max_degrees = 0;

    
    for i in 0..graph_list.len()
    {
        println!("vertex {}: {:?}", i, graph_list[i]);
        num_degrees = graph_list[i].len();
        println!("Number of degrees: {}", num_degrees);

        num_degree_array.push(num_degrees);

        if num_degrees > max_degrees
        {
            max_degrees = num_degrees;
            
        }

        println!();



    };
    


    //println!("Vector of # of Degrees for Designated Node: {:?}", num_degree_array);


    //setting length to max # of degrees
    //cant have degree count more than max #
    let mut degree_count_vec =  vec![0.0 ; max_degrees +1];
 
    for i in 0..num_degree_array.len()
    {

        degree_count_vec[num_degree_array[i]] =  degree_count_vec[num_degree_array[i]] +1.0;

    }
    
    for i in 0..degree_count_vec.len()
    {
        degree_count_vec[i] = degree_count_vec[i] as f32 / n as f32;
    }

    println!("Degree distribution vector: {:?}", degree_count_vec);
    println!();


    println!("{}", "Vertex Degree Distribution:");
  
    
    for i in 0..degree_count_vec.len()
    {
        println!("For degree {}: {}", i, degree_count_vec[i]);
    }
    



    let mut cumulative_error_for_beta_value = 0.0;
    let mut best_beta = 0.0; 
    let mut current_error;
    let mut diff: f64 = 0.0;
    let mut converted_beta = 0.0;
    let mut smallest_error = 0.0;

    for beta in 1..4000
    {
        converted_beta = beta as f64 * 0.001; 

        //beta should start at 1.001 not 0.001 and go up from there.  beta cannot be 1 or below in power law distribution, otherwise would be linear
        converted_beta = converted_beta + 1.0;

        let mut total = 0.0;
   
       

        for i in 1..degree_count_vec.len()
        {

            
            let mut float_i: f64 = i as f64;
            let pi = 1.0 / float_i.powf(converted_beta) as f64;
            total = total +pi;

        }
        cumulative_error_for_beta_value = 0.0;

        for i in 1..degree_count_vec.len()
        {
            let float_i: f64 = i as f64;
            let pi = 1.0 / float_i.powf(converted_beta) as f64;

            //qi = # of nodes for each degree in data by total # of nodes (estimated probability that one of the nodes will have specific # of degrees)
            //pi = current degree count to the -beta

            // important: divided by total in order to normalize the power distribution we are comparing against (it forces the sum of the probabilities across the set of degree possiblities to be 1)
            let pi = pi / total;

            let qi = degree_count_vec[i] as f64; //count # of nodes with degree value i / n;
            diff = (pi - qi) as f64;


            //absolute value of pi - qi (error), and then sum them up

            current_error = diff.abs() as f64;
        
            cumulative_error_for_beta_value = cumulative_error_for_beta_value + current_error;
            

        }
        

        //if cumulative < smallest error so far for that particular beta
        //tracking best beta value with smallest error to show what model for power law distribution looks like
        //smallest_error will represent with error with lowest beta
        
        if beta == 1
        {
            smallest_error = cumulative_error_for_beta_value;
            best_beta = converted_beta;
            println!("smallest_error: {}", smallest_error);
            println!("best_beta: {}", best_beta);
        }
        else
        {
            if cumulative_error_for_beta_value < smallest_error
            {
                smallest_error = cumulative_error_for_beta_value;
                best_beta = converted_beta;
                println!("smallest_error: {}", smallest_error);
                println!("best_beta: {}", best_beta); 
            }
        }   

    }     
    println!("{} {} {} {}", "The best beta value fitted for the power law distribution is:", best_beta, "that produces smallest total error of:", smallest_error);

}  




    
