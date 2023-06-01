export enum TokenType {
  //Literal
  Number,
  Identifier,

  //Keywords
  Let,

  //Grouping * Operators
  Equals,
  OpenParen,
  CloseParen,
  BinaryOperator,
  EOF, //Fim de linha
}

const KEYSWORDS: Record<string, TokenType> = {
  let: TokenType.Let,
};

export interface Token {
  value: string;
  type: TokenType;
}

function token(value = "", type: TokenType): Token {
  return { value, type };
}

function isalpha(src: string) {
  return src.toUpperCase() != src.toLowerCase();
}

function isskippable(str: string) {
  return str == " " || str == "\n" || str == "\t";
}

function isInt(str: string) {
  const c = str.charCodeAt(0);
  const bounds = ["0".charCodeAt(0), "9".charCodeAt(0)];
  return c >= bounds[0] && c <= bounds[1];
}

export function tokenize(sourceCode: string): Token[] {
  const tokens = new Array<Token>();
  const src = sourceCode.split("");

  // criar token ate o fim do arquivo
  while (src.length > 0) {
    if (src[0] == "(") {
      tokens.push(token(src.shift(), TokenType.OpenParen));
    } else if (src[0] == ")") {
      tokens.push(token(src.shift(), TokenType.CloseParen));
    } else if (
      src[0] == "+" ||
      src[0] == "-" ||
      src[0] == "*" ||
      src[0] == "/" ||
      src[0] == "%"
    ) {
      tokens.push(token(src.shift(), TokenType.BinaryOperator));
    } else if (src[0] == "=") {
      tokens.push(token(src.shift(), TokenType.Equals));
    } else {
      //  Tokens de múltiplos caracteres

      //Build Token numero
      if (isInt(src[0])) {
        let num = "";
        while (src.length > 0 && isInt(src[0])) {
          num += src.shift();
        }
        tokens.push(token(num, TokenType.Number));
      } else if (isalpha(src[0])) {
        let ident = "";
        while (src.length > 0 && isalpha(src[0])) {
          ident += src.shift();
        }
        // Verificar palavras reservadas
        const reserved = KEYSWORDS[ident];
        if (typeof reserved == "number") {
          tokens.push(token(ident, reserved));
        } else {
          tokens.push(token(ident, TokenType.Identifier));
        }
      } else if (isskippable(src[0])) {
        src.shift();
      } else {
        console.log(
          "Caractere não reconhecido em source",
          src[0].charCodeAt(0),
          src[0]
        );
        Deno.exit(1);
      }
    }
  }
  tokens.push({ type: TokenType.EOF, value: "EndOfFile" });
  return tokens;
}
