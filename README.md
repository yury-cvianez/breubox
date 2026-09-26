# BreuBox
Caixa preta de sistemas computacionais para auxiliar na investigação de incidentes, cobre toda extensão do funcionamento de um processo observando-o
	 por dentro e por fora para que seja possível relacionar peças e permitir que uma representação ou uma história do sistema consiga ser reconstruída. 

---
Como existem certos problemas que não conseguimos recriá-los, que estão além do que se pode observar com código ou ficar inspecionando logs e mais logs, ter uma caixa preta pode ajudar a reconstruir o acontecimento e isso facilitaria muito a busca pelo causa do erro.

Imagine que isso seja um quebra cabeça, 
mas um quebra cabeça diferente, por que não sabemos como será a imagem final.
Não sabemos antecipadamente:

* onde o problema irá acontecer;
* qual componente estará envolvido;
* qual camada será responsável;
* qual evento será importante;
* nem quais peças precisarão ser relacionadas.

Por isso, o objetivo inicial não é encontrar a resposta.
O objetivo é preservar aquilo que realmente aconteceu.

Então temos o que vou chamar de **"A Evidência"**

---
## **Para que o usuário monte o quebra cabeça, o BreuBox então possui:**

### 1 - Dois fatores principais e tres principais camadas de montagem:
```text
              TEMPO
                ↓
APLICAÇÃO ←── RELAÇÃO ──→ SISTEMA
				│
				↓
			  MUNDO
```

### Fatores Principais:
	Tempo
	Relação

### Camadas de montagem:
    Camada Aplicação:
        Todo código/ferramenta do usuário que está sendo executado na máquina.
        
    Camada Sistema:
        Toda parte operacional da máquina.
        
    Camada Mundo:
        Todo processo externo que se comunica com a aplicação.
	
### 2 - Dentro de cada camada temos os componentes base
Exemplos:
```text
Camada Aplicação:
 Espaço de uso do usuário.
    
    Scripts; 
        python executando no host

    Apps Server;
        gunicorn
    
    Bancos de dados; 
        sql server, postgres, mongodb

    Conteiners; 
        docker

    Observabilidade; 
        grafana, prometheus

    RunTimes; 
        nodejs, jvm

    Automações; 
        jenkins, gitlab

    Schedulers; 
        exemplo: airflow

        
Camada Sistema:
 Funcionamento operacional do host para as aplicações que estão sendo executadas.
    
    Processos;
        pid, ipc, states, threads, fds, memory, cpu, signals
    
    Network;
        socket, tcp/udp, dns, nic, routing, firewall, hops, packet path
        
    Storage;
        filesystem, files, block devices, i/o, lvm, swap, inodes
        
    Memory:
        virtual memory, physical memory, page cache, swap, mmap, oom
        
    Kernel;
        scheduler, syscall, memory, filesystem, drivers, linux security modules
                
    Hardware:
        cpu, ram, disk, nic, firmware, sensors, gpus, tpus


Camada Mundo:
 Saidas, recebimentos, consumo, publicações, sincronizações, resoluções em sistemas externos, aquilo que não esta ao sistema local observado, mas está envolvido.
    
    Computação remota:
        api, services, apps
    
    Network:
        gateways, load balance, remote host
        
    Storage:
        s3, smb, nfs, sftp, remote filesystem
        
    Outras maquinas:
        server, container, vm, maquina fisica
```

## 3 - Modo de Funcionamento:
Cada componente de cada camada faz parte de uma fração de qualquer que seja a imagem final.
Por fim o tempo/relaçao permite que cada peça de cada componente se encaixe.
```text
CAMADAS:
  └─ Camada Aplicação: O que tentou fazer
  └─ Camada Sistema: Como estava a máquina
  └─ Camada Mundo: Qual era o contexto externo

TEMPO:
  └─ Todos os eventos tem timestamp preciso
  └─ Seguir ordem cronológica
  └─ Ver causas → efeitos

RELAÇÃO:
  └─ Processo 18492 → conectar em 10.1.2.3:1433
  └─ Rede entre máquinas: TCP packets
  └─ SQL Server remoto: estava down
```
Todas essas peças ao serem interpretadas com os detalhes, irão reconstruir a imagem do momento do incidente. 
Assim temos então:
#### **"A EVIDÊNCIA"**
```text
						Airflow worker
							  │
							  ▼
						PID 18492
							  │
							  ▼
						TCP socket
							  │
							  ▼
						10.1.2.3:1433
							  │
							  ▼
						request enviada
							  │
							  ▼
						dados começaram a chegar
							  │
							  ▼
						thread deixa de progredir
							  │
							  ├──────────────► CPU permanece baixa
							  │
							  ├──────────────► memória permanece estável
							  │
							  └──────────────► socket permanece ativo
												   │
												   ▼
											task não progride
												   │
												   ▼
											 AIRFLOW TIMEOUT
```

--- 
## **Propriedades:**
* **1 -** Não temos teoria da falha ou partimos do principio de falhas conhecidas, pois não sabemos o que é importante ou o que é a causa, tentamos olhar pro passado, onde possivelmente estara o contexto necessário para o momento.

* **2 -**	De forma assincrona e independente da cooperação do que está sendo observado, no começo de tudo 

* **3 -** Usa-se um buffer circular de tamanho fixo e com registro de perda em casos de buffer cheio

* **4 -** Para cada evento do processo em observação o dado é persistido em lote e registrado em um arquivo de eventos append-only

* **5 -** Além de estar em disco, é enviado ao Limbo (banco rust embarcado)  e sincronizado com um banco (externo ou interno) postgres

* **6 -** A persistencia local em casos de falhas com o banco é gerenciada e limitada pelo block device

* **7 -** Para o tempo temos wall clock + monotonic clock que permite saber o horario do acontecimento e segundos depois do acontecimento permitindo ter relação temporal mais confiavel

* **8 -** O host e o proprio breubox é um processo do sistema, uma entidade observavel

* **9 -** Temos um orçamento de dano, para que em casos criticos o breubox não entre em panico aumentando sua resolução para tentar descobrir o que esta acontecendo

* **10 -** Em caso de extrema anomalia da maquina, breubox prefere ter evidencias incompletas do que fazer parte do problema

* **11 -** Breubox prioriza usar o máximo possível do que o kernel e as primitivas do OS fornece, não queremos depender de linhas de comandos ou ferramentas administrativa.