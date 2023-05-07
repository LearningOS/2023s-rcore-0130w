const MAX_THREADS : usize = 10;
const MAX_RESOURCES : usize = 50;

#[derive(Debug, Copy, Clone)]
/// DeadlockChecker
pub struct DeadlockChecker {
    /// needed resource
    pub need_matrix : [[u32; MAX_RESOURCES]; MAX_THREADS],
    /// available resource
    pub available_vector : [u32; MAX_RESOURCES],
    /// used resource
    pub used_matrix : [[u32; MAX_RESOURCES]; MAX_THREADS],
}

impl DeadlockChecker {
    /// alloc a deadlockchecker
    pub fn new() -> Self{
        Self {
            need_matrix: [[0; MAX_RESOURCES]; MAX_THREADS],
            available_vector: [0; MAX_RESOURCES],
            used_matrix: [[0; MAX_RESOURCES]; MAX_THREADS],
        }
    }

    /// recycle resource of one thread
    pub fn recycle_res(&mut self, tid: usize, res_id: usize) {
        self.available_vector[res_id] +=1;
        self.used_matrix[tid][res_id] -=1;
    }

    /// add resource
    pub fn add_res(&mut self, res_id: usize) {
        self.available_vector[res_id] += 1;
    }
    /// check if status safe or not
    pub fn check(&mut self, thread_count: usize, res_count: usize) -> bool {
        let mut work = self.available_vector;
        let mut finish = [false; MAX_THREADS];

        loop {
            let mut tid = -1;

            for i in 0..thread_count {
                if finish[i] {
                    continue;
                }

                let mut flag = false;
                for j in 0..res_count {
                    if self.need_matrix[i][j] > work[j] {
                        flag = true;
                        break;
                    }
                }

                if flag {
                    continue;
                } else {
                    tid = i as isize;
                }
            }

            if tid != -1 {
                for j in 0..res_count {
                    work[j] += self.used_matrix[tid as usize][j];
                }
                finish[tid as usize] = true
            } else {
                break;
            }
        }

        let mut flag = true;
        for i in 0..thread_count {
            if !finish[i] {
                flag = false;
            }
        }
        flag
    }
}