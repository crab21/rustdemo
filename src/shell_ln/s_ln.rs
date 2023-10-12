use std::{process::Command, io::{stdin, stdout}};

#[test]
fn s_test_m(){
    loop {
        // use the `>` character as the prompt
        // 使用 `>` 作为提示
        // need to explicitly flush this to ensure it prints before read_line
        // 需要显式地刷新它，这样确保它在 read_line 之前打印
        print!("> ");
        // stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        let mut child = Command::new(command)
            .spawn()
            .unwrap();

        // don't accept another command until this one completes
        // 在这个命令处理完之前不再接受新的命令
        child.wait(); 
    }
}