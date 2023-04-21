fn main() {
    // let a=2;
    // let b=31;
    let mut cand: usize=2;//candidate
    let mut prinum: usize =0;//prime number
    let mut prime: Vec<usize>=vec![2];
    let mut judge: usize=0;//0=prime, 1=notprime
    let mut max;
    println!{"{}",2}
    loop{
        judge=0;
        cand+=1;
        max=prime.len(); 
        for n in 1..max+1{
            if cand%prime[n-1]==0{
                judge+=1;
                //println!("add{}",cand);
                break;
            }
        }
        if judge>=1{
            //println!("bye");
        }else{
        prime.push(cand);
        println!("{}",prime[prinum+1]);//if count is prinum(prime number)
        prinum+=1;
        }
        if cand >=2147483647/*a.pow(b)-1*/{
            println!("done");
            break;//exit
        }
        
    }
}
