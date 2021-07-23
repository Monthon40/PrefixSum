extern crate rayon;
use rayon::join;

fn main() {
    let vec1: Vec<u64> = vec![1, 3, 5, 2];
    let vec2: Vec<u64> = vec![3];
    let vec3: Vec<u64> = vec![];
    let vec4: Vec<u64> = vec![23,65,70,4];
    let vec5: Vec<u64> = vec![1,2,3,4,5,6,7,8];

    println!("{:?} => {:?}", &vec1, prefix_sum(&vec1));
    println!("{:?} => {:?}", &vec2, prefix_sum(&vec2));
    println!("{:?} => {:?}", &vec3, prefix_sum(&vec3));
    println!("{:?} => {:?}", &vec4, prefix_sum(&vec4));
    println!("{:?} => {:?}", &vec5, prefix_sum(&vec5));

}

struct Fal {
    left_side: Option<Box<Fal>>, right_side: Option<Box<Fal>>, val: u64
}

fn prefix_sum(xs: &Vec<u64>) -> Vec<u64> {

    fn build(x: &Vec<u64>) -> Fal {
        let long: usize = x.len();
        return match long {
            0 => {
                Fal { left_side: None, right_side: None, val: 0 }
            }
            1 => {
                Fal { left_side: None, right_side: None, val: x[0] }
            }
            _ => {
                let (xx, yy) = x.split_at(long / 2);
                let (fal_left, fal_right) = join(|| build(&xx.clone().to_vec()), || build(&yy.clone().to_vec()), );
                let tleft = fal_left.val.clone();
                let tright = fal_right.val.clone();
                Fal { left_side: Some(Box::new(fal_left)), right_side: Some(Box::new(fal_right)), val: tright + tleft
                }
            }
        }
    }

    fn vector(rr: &Option<Box<Fal>>, cnt: u64) -> Vec<u64> {
        return if rr.is_none() {
            vec![0]
        } else {
            let ttt = rr.as_ref().unwrap();
            if ttt.left_side.is_some() && ttt.right_side.is_some() {
                let (left_vector, right_vector) = join(|| {
                    vector(&ttt.left_side, cnt - ttt.right_side.as_ref().unwrap().val, )
                }, || vector(&ttt.right_side, cnt), );
                help(&left_vector, &right_vector)
            } else {
                vec![cnt]
            }
        }
    }
    let var = build(&xs.clone());
    let highcnt = var.val.clone();
    let vector = vector(&Some(Box::new(var)), highcnt);
    match xs.len() {
        length if length < 1 => return vector, _ => return help(&*vec![0], &*vector),
    }

    fn help(left: &[u64], right: &[u64]) -> Vec<u64> {
        return [left, right].concat();
    }
}

