# BreuBox
Caixa preta de sistemas computacionais para auxiliar na investigação de incidentes, cobre toda extensão do funcionamento de um processo observando-o
	 por dentro e por fora para que seja possível relacionar peças e permitir que uma representação ou uma história do sistema consiga ser reconstruída. 

---
Como existem certos problemas que não conseguimos recriá-los, que estão além do que se pode observar com código ou ficar inspecionando logs e mais logs, ter uma caixa preta pode ajudar a reconstruir o acontecimento e isso facilita muito a busca pelo causa do erro.

Imagine que isso seja um quebra cabeça, 
mas um quebra cabeça diferente, por que não sabemos como será a imagem final.
Não sabemos antecipadamente:

* onde o problema irá acontecer;
* qual componente estará envolvido;
* qual camada será responsável;
* qual evento será importante;
* nem quais peças precisarão ser relacionadas.

O objetivo é preservar aquilo que realmente aconteceu e permitir rever o passado através do que vou chamar de **"A Evidência"**

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
	Tempo:
        wall clock + monotonic clock 

	Relação:
        Comportamento do que acontece no sistema

### Camadas:
    Camada Aplicação:
        Todo código/ferramenta do usuário que está sendo executado na máquina.
        
    Camada Sistema:
        Toda parte operacional da máquina.
        
    Camada Mundo:
        Todo processo externo que se comunica com a aplicação.
	
## 2 - Modo de Funcionamento:
Cada componente de cada camada faz parte de uma fração de qualquer que seja a imagem final.
Por fim o tempo/relaçao permite que cada peça de cada componente se encaixe.
Todas essas peças ao serem interpretadas com os detalhes, irão reconstruir a imagem do momento, como uma
memória operacional do sistema. 
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
* **1 -** Não temos uma teoria da falha e nem partimos do principio de falhas conhecidas, pois não sabemos o que é importante ou o que é a causa,.

* **2 -** Funciona de forma assincrona e independente da cooperação do que está sendo observado.

* **3 -** O host e o proprio breubox é um processo do sistema, uma entidade observavel

* **4 -** Temos um orçamento de dano, para que em casos criticos o breubox não entre em panico aumentando sua resolução para tentar descobrir o que esta acontecendo

* **5 -** Em caso de extrema anomalia da maquina, breubox prefere ter evidencias incompletas do que fazer parte do problema

* **6 -** Breubox prioriza usar o máximo possível do que o kernel e as primitivas do OS fornece, não queremos depender de linhas de comandos ou ferramentas administrativa.

---
## **Primeira Versão 0.0.1:**

### Adaptadores:
	
* Runtime observer: python

* Scheduler: airflow

* Databases: postgres, sqlserver
    