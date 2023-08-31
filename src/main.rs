use std::{vec, cmp::min};
//use bitvec::prelude::*;
//use std::io;
use rand::Rng;

macro_rules! arrsize {
    () => {
        256
    };
}


fn createrandomgrid() -> [[i8;arrsize!()];arrsize!()] 
{
    let mut arr:[[i8;arrsize!()];arrsize!()] = [[0;arrsize!()];arrsize!()];
    let mut rng = rand::thread_rng();
    let mut x: bool = false;
    for i in 0..(arr.len())
    {
        for j in 0..(arr[0].len()) 
        {
            x = rng.gen_bool(0.025);
            if i==j {arr[i][j]=0}
            else if j > i && x {arr[i][j] = rng.gen_range(0..127);}
            else if j < i && x {arr[i][j] = arr[j][i];}
            else {arr[i][j] = -1;}
        }
    }
    //println!("{:?}", arr);
    return arr;
}

fn nextnode(dist:[i16;arrsize!()],explored:[u8;arrsize!()]) -> i16
{
    let mut bestindex: i16 = 0;
    let mut best: i16 = i16::MAX;
    let mut skpcnt: i16 = 0;
    for index in 0..(arrsize!() as u16)
        {
            if explored[index as usize] < 1 {skpcnt+=1;}
            else if dist[index as usize] < best {bestindex = index as i16; best = dist[index as usize]}
            else{skpcnt+=1;}
        }
        if skpcnt < arrsize!() {return bestindex}
        else {return -1}
        
}

fn dijkstra(weights: [[i8; arrsize!()]; arrsize!()], start: u8) -> [i16;arrsize!()]
{
    const MAXSIZ:i16 = i16::MAX-15000;
    let mut dist:[i16;arrsize!()] = [MAXSIZ;arrsize!()];
    let mut explored:[u8;arrsize!()] = [1;arrsize!()];
    let mut checker:i16 = 0;
    dist[start as usize] = 0;
    explored[start as usize] = 0;
    let mut index: u8 = start;
    while explored.iter().sum::<u8>() > 0 
        {
            for i in 0..arrsize!()
            {
                if weights[index as usize][i as usize] <= 0 {}
                else 
                {
                    dist[i] = min(dist[i], weights[index as usize][i as usize] as i16 + dist[index as usize]);
                    
                }
            }
            explored[index as usize] = 0;
            checker = nextnode(dist, explored);
            if checker == -1 {break}
            else {index = checker as u8}
        }
        for i in 0..dist.len() {if dist[i] == MAXSIZ {dist[i] = -1}}
        return dist
    
}

fn main() {
    let weights: [[i8; arrsize!()]; arrsize!()] = createrandomgrid();
    let distancearray:[i16;arrsize!()] = dijkstra(weights,0);
    println!("{:?}", distancearray);
    
}
