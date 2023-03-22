pub struct Solution {}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
struct People {
    score: i32,
    age: i32,
}

impl Ord for People {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score < other.score {
            self.score.cmp(&other.score)
        } else {
            return self.age.cmp(&other.age);
        }
    }
}

impl People{
    fn is_confused(&self,other:&Self)->bool{
        if self.score>other.score&&self.age<other.age||self.score<other.score&&self.age<other.age{
            return true;
        }
        false        
    }
}

fn generatePeopleVec(scores: Vec<i32>, ages: Vec<i32>) -> Vec<People> {
    let mut res = vec![];
    for (index, score) in scores.iter().enumerate() {
        res.push(People {
            score: *score,
            age: ages[index],
        })
    }
    res
}

fn getMaxValue(v:Vec<i32>)->i32{
    let mut res=0;
    for item in v{
        if res<item{
            res=item;
        }
    }
    res
}

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let n=scores.len();
        let mut peopleVec = generatePeopleVec(scores, ages);
        peopleVec.sort();
        // println!("perpleVec = {:?}",peopleVec);
        
        let mut dp=Vec::with_capacity(n);
        for i in 0..n{
            dp.push(peopleVec[i].score);
        }        

        for i in 1..n{
            for j in 0..i{                
                if !peopleVec[i].is_confused(&peopleVec[j]){
                    dp[i]=dp[i].max(dp[j]+peopleVec[i].score);
                }
            }
        }

        // println!("dp = {:?}",dp);

        getMaxValue(dp)
    }
}

#[test]
fn do_some(){
    let scores = vec![100, 50, 50, 10, 15];
    let ages = vec![1, 2, 3, 4, 5];
    let mut people_vec=generatePeopleVec(scores, ages);
    println!("{:?}",people_vec);
    people_vec.sort();
    println!("{:?}",people_vec);
}

#[test]
fn test() {
    let input1_scores = vec![1, 3, 5, 10, 15];
    let input1_ages = vec![1, 2, 3, 4, 5];
    let ans1 = 34;
    let output=Solution::best_team_score(input1_scores, input1_ages);
    println!("{output}");
    assert!(output == ans1);

    let input1_scores = vec![4, 5, 6, 5];
    let input1_ages = vec![2, 1, 2, 1];
    let ans1 = 16;
    let output=Solution::best_team_score(input1_scores, input1_ages);
    println!("{output}");
    assert!(output == ans1);

    let input1_scores = vec![1, 2, 3, 5];
    let input1_ages = vec![8, 9, 10, 1];
    let ans1 = 6;
    let output=Solution::best_team_score(input1_scores, input1_ages);
    println!("{output}");
    assert!(output == ans1);
}
