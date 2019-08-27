extern crate csv;
extern crate time;

use rayon::prelude::*;
use std::f64;

use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;

//this is just for parsing cities
struct City {
    lon:f64,
    lat:f64,
    pop:i32,
}

//we use a struct of vec, to be sure that our code is ugly
struct Cities {
    indic: Vec<i32>,
    edge: Vec<i32>,
    lon: Vec<f64>,
    sinlat: Vec<f64>,
    coslat: Vec<f64>,
    cost: Vec<f64>,
}

fn read_some_cities() -> Result<Vec<City>, Box<dyn Error>> {
    let mut cities_list: Vec<City> = Vec::new();
    let mut reader = csv::Reader::from_reader(io::stdin());
    for record in reader.records() {
        let record = record?;
        let pop: i32 = record[14].parse().unwrap();
        if pop > 1{
            let city = City {
                lon: record[19].parse().unwrap(),
                lat: record[20].parse().unwrap(),
                pop: pop,
            };
            cities_list.push(city);
        }
    }
    return Ok(cities_list)
}

fn prim(cities_list: Vec<City>) -> Vec<i32>{
    //initialize everything
    let mut edges: Vec<i32> = vec![-1; 2*cities_list.len() -2];
    let mut lat = vec![-1.0; cities_list.len()];
    let mut q = Cities {
                    indic: vec![-1; cities_list.len()],
                    edge:  vec![-1; cities_list.len()],
                    lon: vec![-1.0; cities_list.len()],
                    sinlat: vec![-1.0; cities_list.len()],
                    coslat: vec![-1.0; cities_list.len()],
                    cost: vec![-1.0; cities_list.len()],
    }; 

    //more initializations
    for i in 0..cities_list.len() {
        q.indic[i] = i as i32;
        q.edge[i] = 0;
        //we need to convert the angles to radians
        q.lon[i] = cities_list[i].lon*f64::consts::PI/180.0;
        lat[i] = cities_list[i].lat*f64::consts::PI/180.0;
        q.sinlat[i] = lat[i].sin();
        q.coslat[i] = lat[i].cos();
        q.cost[i] = f64::INFINITY;
    }

    //we select the first city as the root of the spanning tree
    let mut f_indic = 0;
    let mut f_lon = q.lon[0];
    let mut f_sinlat = lat[0].sin();
    let mut f_coslat = lat[0].cos();
    //then we remove it from the list of not connected cities
    q.indic.swap_remove(0); 
    q.edge.swap_remove(0); 
    q.lon.swap_remove(0); 
    q.sinlat.swap_remove(0); 
    q.coslat.swap_remove(0); 
    q.cost.swap_remove(0); 

    let mut total_cost = 0f64;
    //add cities to the tree until they are all in the tree
    for c in 0..cities_list.len()-1 {
        //wew
        let tuple = (0..q.cost.len()).into_par_iter()
          .zip(q.cost.par_iter_mut())
          .zip(q.edge.par_iter_mut())
          .zip(q.lon.par_iter())
          .zip(q.sinlat.par_iter())
          .zip(q.coslat.par_iter())
          .map( |(((((i,cost),edge),lon),sinlat),coslat)| 
                {
                    //compute the distance to the newly added city
                    let new_cost = (f_sinlat*sinlat + (f_lon - lon).cos() * f_coslat * coslat).acos(); 
                    //if needed we update this city
                    if new_cost < *cost {
                        //this is the minimal distance to the tree from this city
                        *cost = new_cost;
                        //this is the id of the city in the tree this city should be connected to
                        *edge = f_indic;
                    }
               (i, *cost)
                }
                )
          //we find the closest city to the tree (its id and the cost of the new link)
        .reduce_with(|a, b| 
                     if b.1 < a.1 {
                         b
                     }
                     else {
                         a
                     }
        );
        
        let min_id = tuple.unwrap().0;
        //multiply by 6378 to get the real distance in km
        total_cost += 6378.0*tuple.unwrap().1;
        //add the new edge found to the graph
        edges[2*c] = q.indic[min_id];
        edges[2*c+1] = q.edge[min_id];

        //the city found is now the newest city added to the tree
        f_indic = q.indic[min_id];
        f_lon = q.lon[min_id];
        f_sinlat = q.sinlat[min_id];
        f_coslat = q.coslat[min_id];

        //we remove the new city from the list of free cities
        q.indic.swap_remove(min_id); 
        q.edge.swap_remove(min_id); 
        q.lon.swap_remove(min_id); 
        q.sinlat.swap_remove(min_id); 
        q.coslat.swap_remove(min_id); 
        q.cost.swap_remove(min_id); 
    }
    println!("the graph is : {:.1}km long", total_cost);
    return edges
}
    
    
fn main() {
    //read some cities from stdin
    let cities: Vec<City> = read_some_cities().unwrap();
    //write them to a file
    let mut buffer = File::create("resuCities.dat").unwrap();
    for c in cities.iter() {
        write!(&mut buffer,"{} {} {}\n", c.pop,c.lon, c.lat).unwrap();
    }

    let tic = time::get_time();
    //find the minimum spaning tree with prim's algorithm
    let edges = prim(cities);
    let toc= time::get_time();
    println!("time : {:?}",toc-tic);

    //write the edges of the minimum spanning tree to file
    let mut buffer2 = File::create("resuGraph.dat").unwrap();
    for i in 0..edges.len()/2 {
        write!(&mut buffer2,"{} {}\n",edges[2*i], edges[2*i+1]).unwrap();
    }
}
