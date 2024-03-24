use std::io;

struct Processo {
    pid: i32,
    men_size: i32,
    time_execution: i32,
}
struct Pilha {
    processos: Vec<Processo>,
    next_pid: i32,
}
impl Pilha {
    fn new() -> Pilha {
        Pilha {
            processos: Vec::new(),
            next_pid: 0,
        }
    }

    fn push(&mut self, men_size: i32, time_execution: i32) {
        let processo = Processo {
            pid: self.next_pid,
            men_size,
            time_execution,
        };
        self.processos.push(processo);
        self.next_pid += 1;
    }

    fn print_pids(&self) {
        for processo in &self.processos {
            println!("{}", processo.pid);
        }
    }
}

fn main() {
    use clearscreen::clear;

    let mut pilha: Pilha = Pilha::new();
    let mut input_line = String::new(); //Serve para converter os valores que o usuário digitar de String para inteiro

    //obtem os inputs do usuário
    println!("Digite o número de processos a serem executados");
    io::stdin()
        .read_line(&mut input_line)
        .expect("O valor digitado é inválido");
    let num_processos: i32 = input_line.trim().parse().expect("Valor inválido");
    input_line.clear();

    for i in 0..num_processos {
        let mut tempo_execucao;
        let mut memoria: i32;
        loop {
            println!("Quantidade de memória (MB) a ser alocada para o processo  {}: ",i + 1);
            input_line.clear();
            io::stdin()
                .read_line(&mut input_line)
                .expect("Falha ao ler a entrada");
            memoria = input_line.trim().parse().expect("Valor inválido");

            if (memoria > 0) {break;} else {print!("O valor é muito baixo.");}
        }
        loop {
            println!("Digite o valor do tempo de execução (S) para o processo {}: ",i + 1);
            input_line.clear();
            io::stdin()
                .read_line(&mut input_line)
                .expect("Falha ao ler a entrada");
            tempo_execucao = input_line.trim().parse().expect("Valor inválido");

            if tempo_execucao >= 30 && tempo_execucao <= 90 {break;} else {println!("O TEMPO DE EXECUÇÃO É INVÁLIDO")}
        }
        //adiciona o processo na pilha
        pilha.push(memoria, tempo_execucao)
    }
    //limpa a tela
    clear();
    //exibe o id dos processos a serem executados
    println!("PIDs dos processos a serem executados");
    pilha.print_pids();

    //execução da pilha

    for processo in pilha.processos {
        let mut tempo = processo.time_execution;
        println!("===== Execução iniciada ================");
        println!("===== Processo sendo executado: {} ======", processo.pid);

        while tempo > 0 {
            print!(" {} ", tempo);

            tempo -= 1;
        }
        println!("");
        println!("===== Execução do processo encerrada ===");
        println!("");
        println!("");
    }
}
