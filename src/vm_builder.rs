use std::process::Command;

trait RVMCode {
    fn load(&self) -> Vec<u8>;
}

struct RVMAseembly {
    path: String,
}

//TODO RVMBinary struct (and implement RVMCode for it) to load binary files directly without assembling
//GODDAMN! 개피곤.
//솔직히 아직 테스트 한번도 안해봄! 굴려보지도 않음! 오버킬같음! 자신감과 불안감이 공존하는 행복한 코딩생활
// 다 뇌로는 돌려봤는데 멀정히 작동함 근데 솔직히 아직까지 모르겠음
// 런타임 에러나면 하나하나 고쳐봐야지뭐;

impl RVMCode for RVMAseembly {
    fn load(&self) -> Vec<u8> {
        let output = Command::new("RVA")
            .arg(&self.path)
            .arg("--stdout")
            .output()
            .expect("Failed to run assembler");

        if !output.status.success() {
            panic!("Assembly failed");
        }

        output.stdout
    }
}

struct VmBuilder {}
