// whith IA

import scala.collection.mutable
import java.security.MessageDigest
import java.time.LocalDateTime
import scala.util.Random

class Pasta(val nome: String, val caminho: String) {
  val id: String = gerarId()
  var quantidade: Int = 0
  var valor: Double = 0.0

  private def gerarId(): String = {
    Random.alphanumeric.take(16).mkString
  }

  override def toString: String = {
    s"Pasta(nome=$nome, caminho=$caminho, quantidade=$quantidade, valor=$valor, id=$id)"
  }
}

class Transacao(val pastaId: String, val quantidade: Int, val valor: Double, val timestamp: LocalDateTime) {
  val id: String = gerarIdTransacao()

  private def gerarIdTransacao(): String = {
    Random.alphanumeric.take(16).mkString
  }

  override def toString: String = {
    s"Transacao(pastaId=$pastaId, quantidade=$quantidade, valor=$valor, timestamp=$timestamp, id=$id)"
  }
}

class Bloco(
  val index: Int,
  val transacoes: List[Transacao],
  val hashAnterior: String,
  val timestamp: LocalDateTime
) {
  val hash: String = calcularHash()

  def calcularHash(): String = {
    val input = s"$index$hashAnterior$timestamp${transacoes.map(_.id).mkString}"
    val digest = MessageDigest.getInstance("SHA-256")
    val hashBytes = digest.digest(input.getBytes("UTF-8"))
    hashBytes.map("%02x".format(_)).mkString
  }

  override def toString: String = {
    s"Bloco(index=$index, hash=$hash, hashAnterior=$hashAnterior, transacoes=${transacoes.length}, timestamp=$timestamp)"
  }
}

class Blockchain {
  private var blocos = new mutable.ListBuffer[Bloco]()
  private var transacoesAtuais = new mutable.ListBuffer[Transacao]()
  private var pastas = new mutable.Map[String, Pasta]()

  def criarBlocoGenese(): Unit = {
    val blocoGenese = new Bloco(0, List(), "0", LocalDateTime.now())
    blocos += blocoGenese
    println("Bloco gênese criado!")
  }

  def registrarPasta(pasta: Pasta): Unit = {
    pastas(pasta.id) = pasta
    println(s"Pasta registrada: $pasta")
  }

  def adicionarTransacao(transacao: Transacao): Boolean = {
    if (pastas.contains(transacao.pastaId)) {
      transacoesAtuais += transacao
      println(s"Transação adicionada: $transacao")
      true
    } else {
      println("Erro: Pasta não encontrada!")
      false
    }
  }

  def criarBloco(): Unit = {
    if (transacoesAtuais.isEmpty) {
      println("Nenhuma transação para criar bloco!")
      return
    }

    val hashAnterior = if (blocos.isEmpty) "0" else blocos.last.hash
    val novoBloco = new Bloco(
      blocos.length,
      transacoesAtuais.toList,
      hashAnterior,
      LocalDateTime.now()
    )

    blocos += novoBloco
    transacoesAtuais.clear()
    println(s"Novo bloco criado e adicionado à blockchain!")
  }

  def verificarIntegridade(): Boolean = {
    for (i <- 1 until blocos.length) {
      val blocoAtual = blocos(i)
      val blocoAnterior = blocos(i - 1)

      if (blocoAtual.hashAnterior != blocoAnterior.hash) {
        println(s"Erro: Integridade violada no bloco $i!")
        return false
      }

      val hashCalculado = blocoAtual.calcularHash()
      if (blocoAtual.hash != hashCalculado) {
        println(s"Erro: Hash inválido no bloco $i!")
        return false
      }
    }
    true
  }

  def exibirBlockchain(): Unit = {
    for (bloco <- blocos) {
      println(bloco)
      println(s"Transações:")
      for (transacao <- bloco.transacoes) {
        println(s"  - $transacao")
      }
      println()
    }
  }

  def exibirPastas(): Unit = {
    println("\n=== PASTAS REGISTRADAS ===")
    for ((id, pasta) <- pastas) {
      println(pasta)
    }
  }

  def obterSaldoPasta(pastaId: String): Int = {
    var saldo = 0
    for (bloco <- blocos) {
      for (transacao <- bloco.transacoes) {
        if (transacao.pastaId == pastaId) {
          saldo += transacao.quantidade
        }
      }
    }
    saldo
  }

  def obterTotalTransacoes(): Int = {
    var total = 0
    for (bloco <- blocos) {
      total += bloco.transacoes.length
    }
    total
  }

  def obterTotalBlocos(): Int = {
    blocos.length
  }
}

object Main {
  def main(args: Array[String]): Unit = {
    val blockchain = new Blockchain()
    blockchain.criarBlocoGenese()

    val pasta1 = new Pasta("Pasta 1", "/home/user/pasta1")
    val pasta2 = new Pasta("Pasta 2", "/home/user/pasta2")
    val pasta3 = new Pasta("Pasta 3", "/home/user/pasta3")

    blockchain.registrarPasta(pasta1)
    blockchain.registrarPasta(pasta2)
    blockchain.registrarPasta(pasta3)

    blockchain.adicionarTransacao(new Transacao(pasta1.id, 100, 10.0, LocalDateTime.now()))
    blockchain.adicionarTransacao(new Transacao(pasta2.id, 50, 5.0, LocalDateTime.now()))
    blockchain.criarBloco()

    blockchain.adicionarTransacao(new Transacao(pasta3.id, 75, 7.5, LocalDateTime.now()))
    blockchain.adicionarTransacao(new Transacao(pasta1.id, 25, 2.5, LocalDateTime.now()))
    blockchain.criarBloco()

    blockchain.adicionarTransacao(new Transacao(pasta2.id, 30, 3.0, LocalDateTime.now()))
    blockchain.criarBloco()

    blockchain.verificarIntegridade()
    blockchain.exibirBlockchain()
    blockchain.exibirPastas()

    println(s"\nSaldo da Pasta 1: ${blockchain.obterSaldoPasta(pasta1.id)}")
    println(s"Saldo da Pasta 2: ${blockchain.obterSaldoPasta(pasta2.id)}")
    println(s"Saldo da Pasta 3: ${blockchain.obterSaldoPasta(pasta3.id)}")
    
    println(s"\nTotal de blocos: ${blockchain.obterTotalBlocos()}")
    println(s"Total de transações: ${blockchain.obterTotalTransacoes()}")
  }
}
