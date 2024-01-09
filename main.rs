use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum TokenType {
    ARROW,
    AND,
    ASSIGN,
    ASTERISK,
    CARET,
    COLON,
    COMMA,
    DECREMENT,
    DEFAULT,
    DIVISION,
    DOT,
    EOF,
    EQ,
    FLOAT,
    GT,
    GTE,
    ID,
    INCREMENT,
    INT,
    KEYWORD,
    LBRACE,
    LBRACKET,
    LPAREN,
    LT,
    LTE,
    MODULUS,
    MINUS,
    NE,
    NEGATE,
    OR,
    PLUS,    
    RBRACE,
    RBRACKET,
    RPAREN,
    SEMI,
    SOC,
    STRING
}

const KEYWORDS: [&str; 17] = [
    "let",
    "fn",
    "for",
    "while",
    "if",
    "else",
    "match",
    "True",
    "False",
    "None",
    "class",
    "parent",
    "rn",
    "break",
    "continue",
    "use",
    "as"
];

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    token_value: String
}

struct Lexer {
    code: String,
    current_char: Option<char>,
    line: String,
    position: usize,
}

impl Lexer {
    fn new(code: String) -> Self {
        Self { code, line: String::new(), current_char: None, position: 0 }
    }

    fn advance(&mut self) {
        // move to the next char
		self.current_char = self.line.chars().nth(self.position);
        self.position += 1;
    }

    fn lex(&mut self) -> Vec<Token> {
        let code = self.code.clone();
        let mut tokens: Vec<Token> = Vec::new();
        
        for line in code.lines() {
            //  init position to zero
            self.position = 0;
            self.line = line.to_string();

            //set current char
            self.advance();

            //match char
            while !self.current_char.is_none() {
                let chr = self.current_char.unwrap();

                match chr {
                    _ if chr.is_alphabetic() => {
                        tokens.push(self.get_identifier());
                    },
                    '_' => {
                        tokens.push(self.get_identifier());
                    },
                    _ if chr.is_numeric() => {
                        tokens.push(self.get_number());
                    },
                    '#' => {
                        self.skip_comment();
                        continue;
                    },
                    '\'' | '"' => {
                        tokens.push(self.get_string());
                    }
                    _ if chr.is_whitespace() => {
                        self.skip_whitespace();
                        continue;
                    },
                    '(' => {
                        tokens.push(
                            Token {token_type: TokenType::LPAREN, token_value: "(".to_string()}
                        );
                        self.advance();
                    },
                    ')' => {
                        tokens.push(
                            Token {token_type: TokenType::RPAREN, token_value: ")".to_string()}
                        );
                        self.advance();
                    },
                    ',' => {
                        tokens.push(
                            Token {token_type: TokenType::COMMA, token_value: ",".to_string()}
                        );
                        self.advance();
                    },
                    ':' => {
                        tokens.push(
                            Token {token_type: TokenType::COLON, token_value: ":".to_string()}
                        );
                        self.advance();
                    },
                    ';' => {
                        tokens.push(
                            Token {token_type: TokenType::SEMI, token_value: ";".to_string()}
                        );
                        self.advance();
                    },
                    '>' => {
                        if self.peek() == Some('=') {
                            tokens.push(
                                Token {token_type: TokenType::GTE, token_value: ">=".to_string()}
                            );
                            self.advance();
                            self.advance();
                        } else {
                            tokens.push(
                                Token {token_type: TokenType::GT, token_value: ">".to_string()}
                            );
                            self.advance();
                        }
                    },
                    '<' => {
                        if self.peek() == Some('=') {
                            tokens.push(
                                Token {token_type: TokenType::LTE, token_value: "<=".to_string()}
                            );
                            self.advance();
                            self.advance();
                        } else {
                            tokens.push(
                                Token {token_type: TokenType::LT, token_value: "<".to_string()}
                            );
                            self.advance();
                        }
                    },
                    '[' => {
                        tokens.push(
                            Token {token_type: TokenType::LBRACKET, token_value: "[".to_string()}
                        );
                        self.advance();
                    },
                    ']' => {
                        tokens.push(
                            Token {token_type: TokenType::RBRACKET, token_value: "]".to_string()}
                        );
                        self.advance();
                    },
                    '{' => {
                        tokens.push(
                            Token {token_type: TokenType::LBRACE, token_value: "{".to_string()}
                        );
                        self.advance();
                    },
                    '}' => {
                        tokens.push(
                            Token {token_type: TokenType::RBRACE, token_value: "}".to_string()}
                        );
                        self.advance();
                    },
                    '.' => {
                        if self.peek() == Some('.') {
                            tokens.push(
                                Token {token_type: TokenType::DEFAULT, token_value: "..".to_string()}
                            );
                            self.advance();
                            self.advance();
                        } else {
                            tokens.push(
                                Token {token_type: TokenType::DOT, token_value: ".".to_string()}
                            );
                            self.advance();
                        }
                    },
                    '+' => {
                        if self.peek() == Some('+') {
                            tokens.push(
                                Token {token_type: TokenType::INCREMENT, token_value: "++".to_string()}
                            );
                            self.advance();
                            self.advance();
                        } else {
                            tokens.push(
                                Token {token_type: TokenType::PLUS, token_value: "+".to_string()}
                            );
                            self.advance();
                        }
                    },
                    '-' => {
                        if self.peek() == Some('-') {
                            tokens.push(
                                Token {token_type: TokenType::DECREMENT, token_value: "--".to_string()}
                            );
                            self.advance();
                            self.advance();
                        } else {
                            tokens.push(
                                Token {token_type: TokenType::MINUS, token_value: "-".to_string()}
                            );
                            self.advance();
                        }
                    },
                    '*' => {
                        tokens.push(
                            Token {token_type: TokenType::ASTERISK, token_value: "*".to_string()}
                        );
                        self.advance();
                    },
                    '^' => {
                        tokens.push(
                            Token {token_type: TokenType::CARET, token_value: "^".to_string()}
                        );
                        self.advance();
                    },
                    '/' => {
                        tokens.push(
                            Token {token_type: TokenType::DIVISION, token_value: "/".to_string()}
                        );
                        self.advance();
                    },
                    '%' => {
                        tokens.push(
                            Token {token_type: TokenType::MODULUS, token_value: "%".to_string()}
                        );
                        self.advance();
                    },
                    '=' => {
                        if self.peek() == Some('=') {
                            tokens.push(
                                Token {token_type: TokenType::EQ, token_value: "==".to_string()}
                            );
                            self.advance();
                        } else if self.peek() == Some('>') {
                            tokens.push(
                                Token {token_type: TokenType::ARROW, token_value: "=>".to_string()}
                            );
                            self.advance();
                        } else {
                            tokens.push(
                                Token {token_type: TokenType::ASSIGN, token_value: "=".to_string()}
                            );
                        }
                        self.advance();
                    },
                    '!' => {
                        if self.peek() == Some('=') {
                            tokens.push(
                                Token {token_type: TokenType::NE, token_value: "!=".to_string()}
                            );
                            self.advance();
                            self.advance();
                        } else {
                            tokens.push(
                                Token {token_type: TokenType::NEGATE, token_value: "!".to_string()}
                            );
                            self.advance();
                        }
                    },
                    '&' => {
                        tokens.push(
                            Token {token_type: TokenType::AND, token_value: "&".to_string()}
                        );
                        self.advance();
                    },
                    '|' => {
                        tokens.push(
                            Token {token_type: TokenType::OR, token_value: "|".to_string()}
                        );
                        self.advance();
                    },
                    _ => {
                        println!("SyntaxError: Unknown Character\nline > {}\nCharacter: '{}'", self.line, chr);
                        std::process::exit(1);
                    }
                }
            }
        }
        tokens.push(Token {token_type: TokenType::EOF, token_value: String::from("EOF")});
        tokens        
    }

    fn get_string(&mut self) -> Token {
        let mut result = String::new();
        let mut escape = false;
        let used = self.current_char;

        let mut escape_chars: HashMap<char, char> = HashMap::new();
        escape_chars.insert('n', '\n');
        escape_chars.insert('t', '\t');
        escape_chars.insert('"', '\"');
        escape_chars.insert('\\', '\\');

        self.advance();

        while !self.current_char.is_none() && (self.current_char != used || escape) {
            if escape == true {
                let chr: char = match escape_chars.get(&self.current_char.unwrap()) {
                    Some(c) => *c,
                    None => self.current_char.unwrap()
                };
                result.push(chr);
                escape = false;
            } else {
                if self.current_char == Some('\\') {
                    escape = true;
                } else {
                    result.push(self.current_char.unwrap());
                }
            }
            self.advance();
        }

        self.advance();
        Token { token_type: TokenType::STRING, token_value: result }
    }

    fn peek(&mut self) -> Option<char> {
       self.line.chars().nth(self.position)
    }
    
    fn get_number(&mut self) -> Token {
        let mut result = String::new();
        let mut dot_count = 0;

        while !self.current_char.is_none() && self.current_char.unwrap().is_numeric() || self.current_char == Some('.') {
            if self.current_char == Some('.') {
                if dot_count == 1 {
                    break;
                } else {
                    dot_count += 1;
                }
            }
            result.push(self.current_char.unwrap());
            self.advance();
        }

        if dot_count == 0 {
            Token {
                token_type: TokenType::INT,
                token_value: result
            }
        } else {
            Token {
                token_type: TokenType::FLOAT,
                token_value: result
            }
        }
    }

    fn get_identifier(&mut self) -> Token {
        let mut result = String::new();

        while !self.current_char.is_none() && self.current_char.unwrap().is_alphanumeric() || self.current_char == Some('_') {
            result.push(self.current_char.unwrap());
            self.advance();
        }

        if KEYWORDS.contains(&&result[..]) {
            Token {
                token_type: TokenType::KEYWORD,
                token_value: result
            }
        } else {
            Token {
                token_type: TokenType::ID,
                token_value: result
            }
        }
    }

    fn skip_comment(&mut self) {
        match self.current_char.unwrap() {
            '#' => {
                while !self.current_char.is_none() && self.current_char != Some('\n') {
                    self.advance();
                }
            }
            _ => ()
        }
            
    }

    fn skip_whitespace(&mut self) {
        while !self.current_char.is_none() && self.current_char.unwrap().is_whitespace() {
			self.advance();
        }
    }
}

#[derive(Debug, Clone)]
enum ASTNode {
    Integer {value: i32},
    Float {value: f64},
    Str { value: String },
    None,
    ID { name: String },
    Bool { value: bool },
    Var { name: Rc<ASTNode>, value: Option<Rc<ASTNode>>},
    PropertyAccess { object: Rc<ASTNode>, property: Rc<ASTNode>},
    Index {object: Rc<ASTNode>, index: Rc<ASTNode>},
    Flow { value: String },

    UnaryOperation { operand: Rc<ASTNode>, operator: String},
    BinaryOperation {left: Rc<ASTNode>, operation: String, right: Rc<ASTNode>},
    ExpressionList {list: Vec<ASTNode>},

    If {condition: Rc<ASTNode>, if_block: Vec<ASTNode>, else_block: Option<Vec<ASTNode>>},
    Match {option: Rc<ASTNode>, cases: Vec<ASTNode>},
    Option { condition: Rc<ASTNode>, block: Vec<ASTNode>},
    Default,

    While {condition: Rc<ASTNode>, body:Vec<ASTNode>},
    For {loop_vars: Vec<ASTNode>, object: Rc<ASTNode>, body:Vec<ASTNode>},

    Function{name: Rc<ASTNode>, parameters: (Option<Vec<ASTNode>>, Option<Vec<ASTNode>>), block: Vec<ASTNode>},
    FunctionCall{ name: Rc<ASTNode>, args: Vec<ASTNode>},
    Return {list: Vec<ASTNode>},
    
    Class { name: Rc<ASTNode>,  parent_classes:Option<Vec<ASTNode>>, block:Vec<ASTNode> },
    Parent { name: Rc<ASTNode>, arguments: Vec<ASTNode> },

    Use {modules: Vec<ASTNode>}
}

struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_token: Token {
                token_type: TokenType::SOC,
                token_value:String::from("SOC")
            }
        }
    }

    fn eat(&mut self, token_type: &TokenType) {
        if self.current_token.token_type == *token_type {
            self.advance();
        } else {
            eprintln!("ParseError: Expected TokenType not found.");
            eprintln!("Expected: {:?}", token_type);
            eprintln!("Found: {:?}", self.current_token);
            eprintln!("Didn't complete parsing: {:?}", &self.tokens);
            std::process::exit(1);
        }
    }

    fn advance(&mut self) {
        self.current_token = self.tokens.pop().unwrap();
    }

    fn parse(&mut self) -> Vec<ASTNode> {
        let result = self.program();

        if self.current_token.token_type != TokenType::EOF {
            println!("Error occured while parsing.");
            std::process::exit(1);
        }

        result
    }

    fn program(&mut self) -> Vec<ASTNode> {
        let mut result = Vec::new();
        self.advance();

        while self.current_token.token_type != TokenType::EOF {
            result.push(self.statement());
        }
        result
    }

    fn statement(&mut self) -> ASTNode {
        match self.current_token.token_type {
            TokenType::KEYWORD => {
                match self.current_token.token_value.as_str() {
					"class" => return self.class_declaration(),
					"fn" => return self.function_declaration(),
                    "while" => return self.while_loop(),
                    "for" => return self.for_loop(),
                    "if" => return self.if_statement(),
                    "match" => return self.match_statement(),
                    "let" => return self.variable_declaration(),
                    "rn" => return self.return_statement(),
                    "parent" => self.parent_initialisation(),
                    "use" => self.use_statement(),
                    _ => return self.expression_statement()
                }
            },
            _ => self.expression_statement()
        }
    }

    fn use_statement(&mut self) -> ASTNode {
        // "use" id_statement (",", id_statement)* ";"
        self.eat(&TokenType::KEYWORD);
        let mut modules: Vec<ASTNode> = vec![];

        modules.push(self.id_statement());

        while self.current_token.token_type == TokenType::COMMA {
            self.eat(&TokenType::COMMA);
            modules.push(self.id_statement());
        }

        self.eat(&TokenType::SEMI);
        return ASTNode::Use {modules};
    }

    fn parent_initialisation(&mut self) -> ASTNode {
        // "parent" id_statement args
        self.eat(&TokenType::KEYWORD);
        let name = self.id_statement();

        self.eat(&TokenType::LPAREN);
        let arguments = self.arguments();
        self.eat(&TokenType::RPAREN);

        return ASTNode::Parent{ name: Rc::new(name), arguments};
    }

    fn return_statement(&mut self) -> ASTNode {
        // "return" expression_list
        self.eat(&TokenType::KEYWORD);
        let mut list: Vec<ASTNode> = vec![];

        if self.current_token.token_type == TokenType::SEMI {
            self.eat(&TokenType::SEMI);
            return ASTNode::Return{ list };
        }

        list.push(self.expression());

        while self.current_token.token_type == TokenType::COMMA {
                self.eat(&TokenType::COMMA);
                list.push(self.expression());
        }
        self.eat(&TokenType::SEMI);

        return ASTNode::Return{ list };
    }

    fn match_statement(&mut self) -> ASTNode {
        // "match" id_statement "{" option, (",", option)* default "}"
        self.eat(&TokenType::KEYWORD);
        let option = self.id_statement();

        self.eat(&TokenType::LBRACE);
        let cases = self.cases();
        self.eat(&TokenType::RBRACE);

        return ASTNode::Match {option: Rc::new(option), cases};
    }

    fn cases(&mut self) -> Vec<ASTNode> {
        // expression "=>" block
        let mut cases: Vec<ASTNode> = vec![];
        
        let condition = Rc::new(self.expression());
        self.eat(&TokenType::ARROW);
        cases.push(ASTNode::Option {condition, block: self.block()});
        
        while self.current_token.token_type == TokenType::COMMA {
            self.eat(&TokenType::COMMA);

            if self.current_token.token_type == TokenType::DEFAULT {
                cases.push(self.expression());
                break;
            }
            let condition = Rc::new(self.expression());
            self.eat(&TokenType::ARROW);

            cases.push(ASTNode::Option {condition, block: self.block()});
        }

        return cases;
    }

    fn if_statement(&mut self) -> ASTNode {
        // "if" "(" expression ")" block else_clause
        self.eat(&TokenType::KEYWORD);
        
        self.eat(&TokenType::LPAREN);
        let condition = self.expression();
        self.eat(&TokenType::RPAREN);
        
        let if_block = self.block();

        let else_block: Option<Vec<ASTNode>> = if
            self.current_token.token_type == TokenType::KEYWORD &&
            self.current_token.token_value == "else"
        {
            self.eat(&TokenType::KEYWORD);
            Some(self.block())
        } else {
            None
        };
        return ASTNode::If {condition: Rc::new(condition), if_block, else_block};
    }

    fn for_loop(&mut self) -> ASTNode {
        // "for" "(" id_statement ":" id_statement (",", id_statement)* ")" block
        self.eat(&TokenType::KEYWORD);
        
        self.eat(&TokenType::LPAREN);
        let obj = self.id_statement();

        self.eat(&TokenType::COLON);
        let mut loop_vars: Vec<ASTNode> = vec![];
        loop_vars.push(self.id_statement());

        while self.current_token.token_type == TokenType::COMMA {
            self.eat(&TokenType::COMMA);
            loop_vars.push(self.id_statement());
        }
        self.eat(&TokenType::RPAREN);

        let body = self.block();

        return ASTNode::For{loop_vars, object: Rc::new(obj), body};
    }

    fn while_loop(&mut self) -> ASTNode {
        // "while" "(" expression ")" block
        self.eat(&TokenType::KEYWORD);
        
        self.eat(&TokenType::LPAREN);
        let condition = self.expression();
        self.eat(&TokenType::RPAREN);

        let body = self.block();

        return ASTNode::While {condition: Rc::new(condition), body};
    }

    fn function_declaration(&mut self) -> ASTNode {
        // "func" id_statement parameters block
        self.eat(&TokenType::KEYWORD);
        
        let name = self.id_statement();
        let parameters = self.parameters();
        let block = self.block();

        return ASTNode::Function{name: Rc::new(name), parameters, block};
    }

    fn parameters(&mut self) -> (Option<Vec<ASTNode>>, Option<Vec<ASTNode>>) {
        // "(" input_parameters (":" output_parameters) ")"
        self.eat(&TokenType::LPAREN);
        let mut in_: Vec<ASTNode> = vec![];        
        let mut out_: Vec<ASTNode> = vec![];

        if self.current_token.token_type == TokenType::RPAREN {
            self.eat(&TokenType::RPAREN);
            return (None, None);
        } else if self.current_token.token_type == TokenType::COLON {
            self.eat(&TokenType::COLON);
            
            if self.current_token.token_type == TokenType::RPAREN {
                self.eat(&TokenType::RPAREN);
                return (None, None);
            }
            out_.push(self.expression());

            while self.current_token.token_type == TokenType::COMMA {
                self.eat(&TokenType::COMMA);
                out_.push(self.expression());
            }

            self.eat(&TokenType::RPAREN);
            return (None, Some(out_));
        }
        
        in_.push(self.id_statement());

        while self.current_token.token_type == TokenType::COMMA {
            self.eat(&TokenType::COMMA);
            in_.push(self.id_statement());
        }
        
        if self.current_token.token_type == TokenType::COLON {
            self.eat(&TokenType::COLON);

            if self.current_token.token_type == TokenType::RPAREN {
                self.eat(&TokenType::RPAREN);
                return (Some(in_), None);
            }
            out_.push(self.expression());

            while self.current_token.token_type == TokenType::COMMA {
                self.eat(&TokenType::COMMA);
                out_.push(self.expression());
            }

            self.eat(&TokenType::RPAREN);
            return (Some(in_), Some(out_));
        }
        self.eat(&TokenType::RPAREN);
        return (Some(in_), None);
    }

    fn class_declaration(&mut self) -> ASTNode {
		// class name parent_classes block
		self.eat(&TokenType::KEYWORD);
		let name = self.id_statement();

		let parent_classes: Option<Vec<ASTNode>> = match self.current_token.token_type {
			TokenType::LPAREN => {
				 Some(self.parent_classes())
			},
			_ => {
				None
			}
		};

        let block = self.block();

        return ASTNode::Class{ name:Rc::new(name), parent_classes, block };
	}

    fn block(&mut self) -> Vec<ASTNode> {
        // "{" statement* "}"
        self.eat(&TokenType::LBRACE);
        let mut statements: Vec<ASTNode> = vec![];
        
        while self.current_token.token_type != TokenType::RBRACE {
            statements.push(self.statement());
        }
        self.eat(&TokenType::RBRACE);

        return statements;
    }

	fn parent_classes(&mut self) -> Vec<ASTNode> {
		// "(" id_statements* ")"
		self.eat(&TokenType::LPAREN);
		let mut list: Vec<ASTNode> = vec![];

		if self.current_token.token_type == TokenType::RPAREN {
            self.eat(&TokenType::RPAREN);
            return list;
        }

        list.push(self.id_statement());

        while self.current_token.token_type == TokenType::COMMA {
            self.eat(&TokenType::COMMA);
            list.push(self.id_statement());
        }

        self.eat(&TokenType::RPAREN);
        return list;
    }
           

    fn variable_declaration(&mut self) -> ASTNode {
        // let name = value;
        // or
        // let name;
        self.eat(&TokenType::KEYWORD);
        let name = self.id_statement();

        if self.current_token.token_type == TokenType::SEMI {
            self.eat(&TokenType::SEMI);
            return ASTNode::Var{ name: Rc::new(name), value: None };
        }
        self.eat(&TokenType::ASSIGN);

        let value = self.expression_statement();
        self.eat(&TokenType::SEMI);
        
        return ASTNode::Var{ name: Rc::new(name), value: Some(Rc::new(value)) };
    }

    fn id_statement(&mut self) -> ASTNode {
        let mut var: ASTNode;
        let name = self.current_token.token_value.clone();
        

        self.eat(&TokenType::ID);
        var = ASTNode::ID{ name };

        while self.current_token.token_type == TokenType::DOT {
            self.eat(&TokenType::DOT);
            let property = Rc::new(
                ASTNode::ID {
                    name: self.current_token.token_value.clone()
                }
            );
            self.eat(&TokenType::ID);
            
            var = ASTNode::PropertyAccess {
                object: Rc::new(var),
                property
            };
        }
        var
    }

    fn expression_statement(&mut self) -> ASTNode {
        return self.expression();
    }

    fn expression(&mut self) -> ASTNode {
        let mut result = self.comparison_expression();
        let mut operation;

        while [TokenType::AND, TokenType::OR].contains(&self.current_token.token_type) {
            if self.current_token.token_type == TokenType::AND {
                operation = "&".to_string();
                self.eat(&TokenType::AND);
            } else {
                operation = "|".to_string();
                self.eat(&TokenType::OR);
            }
            result = ASTNode::BinaryOperation {
                left: Rc::new(result),
                operation,
                right: Rc::new(self.comparison_expression())
            };
        }
        result
    }

    fn comparison_expression(&mut self) -> ASTNode {
        let mut result = self.power_expression();
        let mut operation;

        while [TokenType::LT, TokenType::LTE, TokenType::GT, TokenType::GTE, TokenType::EQ, TokenType::NE]
            .contains(&self.current_token.token_type) {
            match self.current_token.token_type {
                TokenType::LT => {
                    self.eat(&TokenType::LT);
                    operation = "<";
                },
                TokenType::LTE => {
                    self.eat(&TokenType::LTE);
                    operation = "<=";
                },
                TokenType::GT => {
                    self.eat(&TokenType::GT);
                    operation = ">";
                },
                TokenType::GTE => {
                    self.eat(&TokenType::GTE);
                    operation = ">=";
                },
                TokenType::EQ => {
                    self.eat(&TokenType::EQ);
                    operation = "==";
                },
                _ => {
                    self.eat(&TokenType::NE);
                    operation = "!=";
                }
            }
            result = ASTNode::BinaryOperation {
                left: Rc::new(result),
                operation: operation.to_string(),
                right: Rc::new(self.power_expression())
            };
        }
        result
    }

    fn power_expression(&mut self) -> ASTNode {
        let mut result = self.arithmetic_expression();
        let mut operation;

        while [TokenType::MODULUS, TokenType::CARET].contains(&self.current_token.token_type) {
            if self.current_token.token_type == TokenType::MODULUS {
                self.eat(&TokenType::MODULUS);
                operation = "%".to_string();
            } else {
                self.eat(&TokenType::CARET);
                operation = "^".to_string();
            }
            result = ASTNode::BinaryOperation {
                left: Rc::new(result),
                operation,
                right: Rc::new(self.arithmetic_expression())
            };
        }
        result
    }

    fn arithmetic_expression(&mut self) -> ASTNode {
        let mut result = self.term();
        let mut operation;

        while [TokenType::PLUS, TokenType::MINUS].contains(&self.current_token.token_type) {
            if self.current_token.token_type == TokenType::PLUS {
                self.eat(&TokenType::PLUS);
                operation = "+".to_string();
            } else {
                self.eat(&TokenType::MINUS);
                operation = "-".to_string();
            }
            result = ASTNode::BinaryOperation {
                left: Rc::new(result),
                operation,
                right: Rc::new(self.term())
            };
        }
        result
    }

    fn term(&mut self) -> ASTNode {
        let mut result = self.primary();
        let mut operation;

        while [TokenType::ASTERISK, TokenType::DIVISION].contains(&self.current_token.token_type) {
            if self.current_token.token_type == TokenType::ASTERISK {
                self.eat(&TokenType::ASTERISK);
                operation = "*".to_string();
            } else {
                self.eat(&TokenType::DIVISION);
                operation = "/".to_string();
            }
            result = ASTNode::BinaryOperation {
                left: Rc::new(result),
                operation,
                right: Rc::new(self.primary())
            };
        }
        result
    }

    fn primary(&mut self) -> ASTNode {
        if self.current_token.token_type == TokenType::ID {
            let var = self.id_statement();

            if [TokenType::LPAREN, TokenType::LBRACKET, TokenType::INCREMENT, TokenType::DECREMENT]
                .contains(&self.current_token.token_type) {
                return self.factor_suffix(var);
            }
            
            return var;
        } else if self.current_token.token_type == TokenType::INT {
            let value: i32 = self.current_token.token_value.trim().parse()
                .expect(format!("Parse Error: Expected Integer but found\n Value > {}", self.current_token.token_value)
                .as_str());
            self.eat(&TokenType::INT);
            return ASTNode::Integer{value};
        } else if self.current_token.token_type == TokenType::FLOAT {
            let value: f64 = self.current_token.token_value.trim().parse()
                .expect(format!("Parse Error: Expected Float but found\n Value > {}", self.current_token.token_value)
                .as_str());
            self.eat(&TokenType::FLOAT);
            return ASTNode::Float{ value };
        } else if self.current_token.token_type == TokenType::STRING {
            let value = self.current_token.token_value.clone();
            self.eat(&TokenType::STRING);
            return ASTNode::Str {value};
        } else if self.current_token.token_type == TokenType::KEYWORD {
            let value = self.current_token.token_value.clone();
            self.eat(&TokenType::KEYWORD);

            if value == "None" {
				return ASTNode::None;
			} else if value == "True" {
				return ASTNode::Bool { value: true };
			} else if value == "False" {
				return ASTNode::Bool { value: false };
			}
            return ASTNode::Flow {value};
        } else if self.current_token.token_type == TokenType::LPAREN {
            self.eat(&TokenType::LPAREN);
            let expr = self.expression();

            self.eat(&TokenType::RPAREN);
            return expr;
        } else if self.current_token.token_type == TokenType::LBRACKET {
			self.eat(&TokenType::LBRACKET);
            let mut expr_list: Vec<ASTNode> = vec![];

            if self.current_token.token_type == TokenType::RBRACKET {
                self.eat(&TokenType::RBRACKET);
                return ASTNode::ExpressionList{ list: expr_list };
            }
            
            expr_list.push(self.expression());
            while self.current_token.token_type == TokenType::COMMA {
				self.eat(&TokenType::COMMA);
                expr_list.push(self.expression());
            }
            self.eat(&TokenType::RBRACKET);
            return ASTNode::ExpressionList{ list: expr_list };
        } else if self.current_token.token_type == TokenType::DEFAULT {
            self.eat(&TokenType::DEFAULT);
            self.eat(&TokenType::ARROW);
            ASTNode::Option {
                condition: Rc::new(ASTNode::Default),
                block: self.block()
            }
        } else if self.current_token.token_type == TokenType::PLUS {
            self.eat(&TokenType::PLUS);
            ASTNode::UnaryOperation {
                operand: Rc::new(self.expression()),
                operator: "+".to_string()
            }
        } else if self.current_token.token_type == TokenType::MINUS {
            self.eat(&TokenType::MINUS);
            ASTNode::UnaryOperation {
                operand: Rc::new(self.expression()),
                operator: "-".to_string()
            }
        } else if self.current_token.token_type == TokenType::NEGATE {
            self.eat(&TokenType::NEGATE);
            ASTNode::UnaryOperation {
                operand: Rc::new(self.expression()),
                operator: "!".to_string()
            }
        } else {
            println!("ParseError: Unexpected Token");
            println!("Token > {:?}", &self.current_token);
            std::process::exit(1);
        }
        
    }

    fn factor_suffix(&mut self, expression: ASTNode) -> ASTNode {
        match self.current_token.token_type {
            TokenType::LPAREN => {
                self.eat(&TokenType::LPAREN);
                let args = self.arguments();
                self.eat(&TokenType::RPAREN);
                
                return ASTNode::FunctionCall {
                    name: Rc::new(expression),
                    args
                };
            },
            TokenType::LBRACKET => {
                self.eat(&TokenType::LBRACKET);
                let index = Rc::new(self.expression());
                self.eat(&TokenType::RBRACKET);

                return ASTNode::Index {
                    object: Rc::new(expression),
                    index
                };
            },
            TokenType::INCREMENT => {
                self.eat(&TokenType::INCREMENT);
                self.eat(&TokenType::SEMI);
                return ASTNode::UnaryOperation {
                    operand: Rc::new(expression),
                    operator: "++".to_string()
                };
            },
            _ => {
                self.eat(&TokenType::DECREMENT);
                self.eat(&TokenType::SEMI);
                return ASTNode::UnaryOperation {
                    operand: Rc::new(expression),
                    operator: "--".to_string()
                };
            }
                
        }
    }

    fn arguments(&mut self) -> Vec<ASTNode> {
        let mut args: Vec<ASTNode> = Vec::new();
        if self.current_token.token_type == TokenType::RPAREN {
            return args;
        }
        args.push(self.expression());

        while self.current_token.token_type == TokenType::COMMA {
			self.eat(&TokenType::COMMA);
            args.push(self.expression());
        }
        return args;
    }
}

/*
    Integer {value: i32},
    Float {value: f64},
    Str { value: String },
    None,
    ID { name: String },
    Var { name: Rc<ASTNode>, value: Option<Rc<ASTNode>>},
    PropertyAccess { object: Rc<ASTNode>, property: Rc<ASTNode>},
    Index {object: Rc<ASTNode>, index: Rc<ASTNode>},
    Flow { value: String },

    UnaryOperation { operand: Rc<ASTNode>, operator: String},
    BinaryOperation {left: Rc<ASTNode>, operation: String, right: Rc<ASTNode>},
    ExpressionList {list: Vec<ASTNode>},

    If {condition: Rc<ASTNode>, if_block: Vec<ASTNode>, else_block: Option<Vec<ASTNode>>},
    Match {option: Rc<ASTNode>, cases: Vec<ASTNode>},
    Option { condition: Rc<ASTNode>, block: Vec<ASTNode>},
    Default,

    While {condition: Rc<ASTNode>, body:Vec<ASTNode>},
    For {loop_vars: Vec<ASTNode>, object: Rc<ASTNode>, body:Vec<ASTNode>},

    Function{name: Rc<ASTNode>, parameters: (Option<Vec<ASTNode>>, Option<Vec<ASTNode>>), block: Vec<ASTNode>},
    FunctionCall{ name: Rc<ASTNode>, args: Vec<ASTNode>},
    Return {list: Vec<ASTNode>},
    
    Class { name: Rc<ASTNode>,  parent_classes:Option<Vec<ASTNode>>, block:Vec<ASTNode> },
    Parent { name: Rc<ASTNode>, arguments: Vec<ASTNode> },

    Use {modules: Vec<ASTNode>}
*/
#[derive(Debug, Clone)]
enum LazyResult {
	Null, //No return used int
	Int(i32),
	Float(f64),
	Str(String),
	Bool(bool),
	List(Vec<Value>),
	None,      // Used in Mar
	Expression{expr: Rc<ASTNode>},
}

struct Executor {
    ast: Vec<ASTNode>,
    current_scope: HashMap<String, Option<LazyResult>>,
    scopes: Vec<HashMap<String, Option<LazyResult>>>,
    functions: Vec<
					HashMap<
						String,
						(
							(Option<Vec<ASTNode>>, Option<Vec<ASTNode>>),
							Vec<ASTNode>
						)
					>
				>,
    return_value: Option<Value>
}

const BUILTIN_FUNCTIONS: [&str; 2] = [
	"print",
	"println",
];

#[derive(Debug, Clone)]
struct Value {
	int_value: Option<i32>,
	float_value: Option<f64>,
	bool_value: Option<bool>,
	string_value: Option<String>,
	list_value: Option<Vec<Value>>,
	value_type: u8
	/*
	 * 0   - ----- - int
	 * 1   - ----- - float
 	 * 2   - ----- - bool
 	 * 3   - ----- - string
 	 * 4   - ----- - None
 	 * 5   - ----- - list
 	 * 127 - ----- - Undefined
 	 */
}
use std::fmt::Display;
use std::fmt::Formatter;

impl Display for Value {
	fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		let res = match self.value_type {
			0 => format!("{}", self.int_value.unwrap()),
			1 => format!("{}", self.float_value.unwrap()),
			2 => format!("{}", self.bool_value.unwrap()),
			3 => format!("{}", self.string_value.clone().unwrap()),
			4 => format!("None"),
			5 => format!("{:?}", self.list_value.clone().unwrap()),
			127 => {
				 println!("(Int)Undefined Value Type");
				 std::process::exit(1);
			 },
			_ => {
				 println!("(Int)Invalid Value");
				 std::process::exit(1);
			 }
		};
		print!("{res}");
		Ok(())
	}
}
	

impl Executor {
    fn new(ast: Vec<ASTNode>) -> Executor {
        Self {
            ast,
            functions: vec![HashMap::new()],
            scopes: vec![HashMap::new()],
            current_scope: HashMap::new(),
            return_value: None,
        }
    }

    fn execute(&mut self) {
		for statement in self.ast.clone().into_iter() {
			self.execute_statement(statement);
			//println!("{:?}", self.scopes);
		}
    }

    fn execute_statement(&mut self, statement: ASTNode) -> LazyResult {
		match statement {
			ASTNode::Var{name, value} => {
				return self.var_declaration(&name, value);
			},
			ASTNode::Function{name, parameters, block} => {
				return self.func_declaration(name, parameters, block);
			},
			ASTNode::FunctionCall{ref name, args} => {
				return self.func_call(&name, args);
			},
			ASTNode::Return{ list } => {
				return self.rn_statement(list);
			}
			_ => {
				println!(">> {statement:?}");
				return LazyResult::Null;
			},
		}
	}

	fn rn_statement(&mut self, list: Vec<ASTNode>) -> LazyResult {
		if list.len() == 0 {
			self.return_value = Some(self.lazy2_value(LazyResult::None));
		} else if list.len() == 1 {
			let expression: ASTNode = list[0].clone();
			let value = self.evaluate(expression);
			self.return_value = Some(value);
		} else {
			let expressions: Value = Value {
				int_value: None,
				float_value: None,
				bool_value: None,
				string_value: None,
				list_value: Some(
								list
								.into_iter()
								.map(|exp| self.evaluate(exp))
								.collect()
				),
				value_type: 5_u8
			};
			
			self.return_value = Some(expressions);
		}
			
		return LazyResult::Null;
	}

	fn func_call(&mut self, name: &Rc<ASTNode>, args: Vec<ASTNode>) -> LazyResult {
		let func_name: &str = match **name {
			ASTNode::ID{ref name} => {
				name
			},
			_ => {
				println!("Name: {:?}", &name);
				println!("Invalid function name");
				std::process::exit(1);
			}
		};

		if BUILTIN_FUNCTIONS.contains(&func_name) {
			match func_name {
				 "print" | "println" => {
					let mut result = String::new();
					
					for arg in &args {
						let value: String = self.evaluate(arg.clone()).to_string();
						result.push_str(value.as_str());
					}
					return match func_name {
						"print" =>  self.print(result),
						_ =>  self.println(result)
					}
				},
				_ => {
					println!("Builtin Function: {func_name} has not been implemented.");

					return LazyResult::Null;
				}
			}
		} else {
			return self.execute_func(func_name.to_string(), args);
		}
	}

	fn print(&mut self, result: String) -> LazyResult {
		print!("{}", result);
		return LazyResult::Null;
	}
	
	fn println(&mut self, result: String) -> LazyResult {
		println!("{}", result);
		return LazyResult::Null;
	}

	fn evaluate(&mut self, expression: ASTNode) -> Value {
		match expression {
			ASTNode::Integer{value} => {
				Value {
					int_value: Some(value),
					float_value: None,
					bool_value: None,
					string_value: None,
					list_value: None,
					value_type: 0_u8
				}
			},
			ASTNode::Float{value} => {
				Value {
					int_value: None,
					float_value: Some(value),
					bool_value: None,
					string_value: None,
					list_value: None,
					value_type: 1_u8
				}
			},
			ASTNode::Bool{value} => {
				Value {
					int_value: None,
					float_value: None,
					bool_value: Some(value),
					string_value: None,
					list_value: None,
					value_type: 2_u8
				}
			},
			ASTNode::Str{value} => {
				Value {
					int_value: None,
					float_value: None,
					bool_value: None,
					string_value: Some(value),
					list_value: None,
					value_type: 3_u8
				}
			},
			ASTNode::None => {
				Value {
					int_value: None,
					float_value: None,
					bool_value: None,
					string_value: None,
					list_value: None,
					value_type: 4_u8
				}
			},
			ASTNode::ExpressionList {list} => {
				let value: Vec<Value> = list.into_iter().map(|x| self.evaluate(x.clone())).collect();
				Value {
					int_value: None,
					float_value: None,
					bool_value: None,
					string_value: None,
					list_value: Some(value),
					value_type: 5_u8
				}
			},
			ASTNode::ID{ name } => {
				let rn_lazy_val = self.get_variable_value(&name).unwrap();
				
				let rn_value: Value;
				match rn_lazy_val {
					LazyResult::Expression { expr } => {
						//We have an expression to execute
						let expr: &ASTNode = &(*expr.clone());

						rn_value = self.evaluate(expr.clone());
					},
					_ => {
						rn_value = self.lazy2_value(rn_lazy_val);
					}
				}
				return rn_value;
			}
			ASTNode::FunctionCall{ref name, args} => {
				let var = self.func_call(&name, args);

				return self.lazy2_value (var);
			},
			ASTNode::BinaryOperation {ref left, operation, ref right} => {
				return self.evaluate_binary_expression(left.clone(), operation, right.clone());
			},
			ASTNode::UnaryOperation {ref operand, ref operator} => {
				return self.evaluate_unary_expression(operator.to_string(), operand.clone());
			},
			_ => {
				println!("Invalid expression at {expression:#?}");
				std::process::exit(1);
			}
		}
	}

	fn get_variable_value(&mut self, name: &String)-> Option<LazyResult> {
		if self.current_scope.contains_key(name) {
			return self.current_scope.get(name).unwrap().clone();
		}
		self.scopes.reverse();
		
		for scope in &self.scopes.clone() {
			if scope.contains_key(name) {
				self.scopes.reverse();
				return scope.get(name).unwrap().clone();
			}
		}
		println!("RTE: Variable `{name}` not defined");
		std::process::exit(1);
	}

	fn evaluate_binary_expression(&mut self, left:Rc<ASTNode>, operation:String, right:Rc<ASTNode>) -> Value {
		let value = self.evaluate((*left).clone());
		let lazy_left_value = self.value2_lazy(value);

		let value = self.evaluate((*right).clone());
		let lazy_right_value = self.value2_lazy(value);

		match operation.as_str() {
			"+" => {
				match lazy_left_value {
					LazyResult::Int(ll_value) => {
						match lazy_right_value {
							LazyResult::Int(lr_value) => {
								return self.lazy2_value(LazyResult::Int(ll_value + lr_value));
							},
							LazyResult::Float(lr_value) => {
								return self.lazy2_value(LazyResult::Float(ll_value as f64 + lr_value));
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Int + bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Int + Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Int + None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Int + Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Int + Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::Float(ll_value) => {
						match lazy_right_value {
							LazyResult::Int(lr_value) => {
								return self.lazy2_value(LazyResult::Float(ll_value + lr_value as f64));
							},
							LazyResult::Float(lr_value) => {
								return self.lazy2_value(LazyResult::Float(ll_value + lr_value));
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Float + bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Float + Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Float + None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Float + Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Float + Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::Bool(..) => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `bool + Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `bool + Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `bool + bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `bool + Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `bool + None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `bool + Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `bool + Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::Str(mut ll_value) => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `Str + Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `Str + Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Str + bool`");
								std::process::exit(1);
							},
							LazyResult::Str(lr_value) => {
								ll_value.push_str(&lr_value);
								return self.lazy2_value(LazyResult::Str(ll_value));
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Str + None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Str + Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Str + Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::None => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `None + Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `None + Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `None + bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `None + Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `None + None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `None + Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `None + Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::List(mut ll_value) => {
						match lazy_right_value {
							LazyResult::Int(lr_value) => {
								ll_value.push(self.lazy2_value(LazyResult::Int(lr_value)));
								return self.lazy2_value(LazyResult::List(ll_value));
							},
							LazyResult::Float(lr_value) => {
								ll_value.push(self.lazy2_value(LazyResult::Float(lr_value)));
								return self.lazy2_value(LazyResult::List(ll_value));
							},
							LazyResult::Bool(lr_value) => {
								ll_value.push(self.lazy2_value(LazyResult::Bool(lr_value)));
								return self.lazy2_value(LazyResult::List(ll_value));
							},
							LazyResult::Str(lr_value) => {
								ll_value.push(self.lazy2_value(LazyResult::Str(lr_value)));
								return self.lazy2_value(LazyResult::List(ll_value));
							},
							LazyResult::None => {
								ll_value.push(self.lazy2_value(LazyResult::None));
								return self.lazy2_value(LazyResult::List(ll_value));
							},
							LazyResult::List(lr_value) => {
								ll_value.extend(lr_value);
								return self.lazy2_value(LazyResult::List(ll_value));
							}
							_ => {
								println!("RTE: No implementation for `Vector + Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					_ => {
						println!("RTE: No implemention for {lazy_left_value:?} + Type");
						std::process::exit(1);
					}
				}	
			}, 
			
			"-" => {
				match lazy_left_value {
					LazyResult::Int(ll_value) => {
						match lazy_right_value {
							LazyResult::Int(lr_value) => {
								return self.lazy2_value(LazyResult::Int(ll_value - lr_value));
							},
							LazyResult::Float(lr_value) => {
								return self.lazy2_value(LazyResult::Float(ll_value as f64 - lr_value));
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Int - bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Int - Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Int - None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Int - Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Int - Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::Float(ll_value) => {
						match lazy_right_value {
							LazyResult::Int(lr_value) => {
								return self.lazy2_value(LazyResult::Float(ll_value - lr_value as f64));
							},
							LazyResult::Float(lr_value) => {
								return self.lazy2_value(LazyResult::Float(ll_value - lr_value));
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Float - bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Float - Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Float - None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Float - Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Float - Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::Bool(..) => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `bool - Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `bool - Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `bool - bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `bool - Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `bool - None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `bool - Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `bool - Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::Str(..) => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `Str - Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `Str - Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Str - bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Str - Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Str - None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Str - Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Str - Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::None => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `None - Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `None - Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `None - bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `None - Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `None - None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `None - Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `None - Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::List(..) => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `Vector - Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `Vector - Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Vector - Bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Vector - Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Vector - None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Vector - Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Vector - Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					_ => {
						println!("RTE: No implemention for {lazy_left_value:?} - Type");
						std::process::exit(1);
					}
				}	
			}, 
			
			"/" => {
				match lazy_left_value {
					LazyResult::Int(ll_value) => {
						match lazy_right_value {
							LazyResult::Int(lr_value) => {
								return self.lazy2_value(LazyResult::Int(ll_value / lr_value));
							},
							LazyResult::Float(lr_value) => {
								return self.lazy2_value(LazyResult::Float(ll_value as f64 / lr_value));
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Int / bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Int / Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Int / None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Int / Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Int / Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::Float(ll_value) => {
						match lazy_right_value {
							LazyResult::Int(lr_value) => {
								return self.lazy2_value(LazyResult::Float(ll_value / lr_value as f64));
							},
							LazyResult::Float(lr_value) => {
								return self.lazy2_value(LazyResult::Float(ll_value / lr_value));
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Float / bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Float / Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Float / None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Float / Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Float / Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::Bool(..) => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `bool / Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `bool / Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `bool / bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `bool / Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `bool / None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `bool / Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `bool / Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::Str(..) => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `Str / Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `Str / Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Str / bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Str / Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Str / None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Str / Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Str / Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::None => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `None / Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `None / Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `None / bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `None / Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `None / None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `None / Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `None / Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::List(..) => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `Vector / Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `Vector / Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Vector / Bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Vector / Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Vector / None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Vector / Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Vector / Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					_ => {
						println!("RTE: No implemention for {lazy_left_value:?} / Type");
						std::process::exit(1);
					}
				}	
			}, 
			
			"*" => {
				match lazy_left_value {
					LazyResult::Int(ll_value) => {
						match lazy_right_value {
							LazyResult::Int(lr_value) => {
								return self.lazy2_value(LazyResult::Int(ll_value * lr_value));
							},
							LazyResult::Float(lr_value) => {
								return self.lazy2_value(LazyResult::Float(ll_value as f64 * lr_value));
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Int * bool`");
								std::process::exit(1);
							},
							LazyResult::Str(lr_value) => {
								let mut result = String::new();

								for _ in 0..=ll_value-1 {
									result.push_str(&lr_value);
								}
								return self.lazy2_value(LazyResult::Str(result));
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Int * None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Int * Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Int * Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::Float(ll_value) => {
						match lazy_right_value {
							LazyResult::Int(lr_value) => {
								return self.lazy2_value(LazyResult::Float(ll_value * lr_value as f64));
							},
							LazyResult::Float(lr_value) => {
								return self.lazy2_value(LazyResult::Float(ll_value * lr_value));
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Float * bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Float * Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Float * None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Float * Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Float * Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::Bool(..) => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `bool * Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `bool * Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `bool * bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `bool * Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `bool * None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `bool * Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `bool * Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::Str(ll_value) => {
						match lazy_right_value {
							LazyResult::Int(lr_value) => {
								let mut result = String::new();

								for _ in 0..=lr_value-1 {
									result.push_str(&ll_value);
								}
								return self.lazy2_value(LazyResult::Str(result));
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `Str * Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Str * bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Str * Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Str * None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Str * Vector`");
								std::process::exit(1);
							},
							_ => {
								println!("RTE: No implementation for `Str * Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::None => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `None * Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `None * Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `None * bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `None * Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `None * None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `None * Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `None * Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					LazyResult::List(..) => {
						match lazy_right_value {
							LazyResult::Int(..) => {
								println!("RTE: No implementation for `Vector * Int`");
								std::process::exit(1);
							},
							LazyResult::Float(..) => {
								println!("RTE: No implementation for `Vector * Float`");
								std::process::exit(1);
							},
							LazyResult::Bool(..) => {
								println!("RTE: No implementation for `Vector * Bool`");
								std::process::exit(1);
							},
							LazyResult::Str(..) => {
								println!("RTE: No implementation for `Vector * Str`");
								std::process::exit(1);
							},
							LazyResult::None => {
								println!("RTE: No implementation for `Vector * None`");
								std::process::exit(1);
							},
							LazyResult::List(..) => {
								println!("RTE: No implementation for `Vector * Vector`");
								std::process::exit(1);
							}
							_ => {
								println!("RTE: No implementation for `Vector * Type`.\nMay be caused by int.");
								std::process::exit(1);
							}
						}
					},
					_ => {
						println!("RTE: No implemention for {lazy_left_value:?} * Type");
						std::process::exit(1);
					}
				}	
			}, 
			
			_ => {
				println!("(Int) Binary operator not Implemented {operation}");
				std::process::exit(1);
			}
		}
	}
	
	// UnaryOperation { operand: Rc<ASTNode>, operator: String},
	fn evaluate_unary_expression(&mut self, operator: String, operand: Rc<ASTNode>) -> Value {
		match operator.as_str() {
			"!" => {
				match *operand {
					ASTNode::None => {
						Value {
							int_value: None,
							float_value: None,
							bool_value: Some(true),
							string_value: None,
							list_value: None,
							value_type: 2_u8
						}
					},
					ASTNode::Bool { value } => Value {
						int_value: None,
						float_value: None,
						bool_value: Some(!value),
						string_value: None,
						list_value: None,
						value_type: 2_u8
					},
					ASTNode::Integer { value } => Value {
						int_value: Some(!value),
						float_value: None,
						bool_value: None,
						string_value: None,
						list_value: None,
						value_type: 0_u8
					},
					ASTNode::Float {..} => {
						println!("RTE: Cannot apply unary operator `!` to type Float");
						std::process::exit(1);
					},
					ASTNode::Str{..} => {
						println!("RTE: Cannot apply unary operator `!` to type Str");
						std::process::exit(1);
					},
					ASTNode::ExpressionList {ref list} => {
						if list.len() > 0 {
							Value {
								int_value: None,
								float_value: None,
								bool_value: Some(false),
								string_value: None,
								list_value: None,
								value_type: 2_u8
							}
						} else { // (![] == true )        -> True
							Value {
								int_value: None,
								float_value: None,
								bool_value: Some(true),
								string_value: None,
								list_value: None,
								value_type: 2_u8
							}
						}
					},
					_ => {
						println!("(Int) Unary operator `!` not implemented for {operand:?}");
						std::process::exit(1);
					}
				}
			}, 
			"-" => {
				match *operand {
					ASTNode::None => {
						println!("RTE: Cannot apply unary operator `-` to type None.");
						std::process::exit(1);
					},
					ASTNode::Bool { .. } =>  {
						println!("RTE: Cannot apply unary operator `-` to type Bool ");
						std::process::exit(1);
					},
					ASTNode::Integer { value } => Value {
						int_value: Some(-value),
						float_value: None,
						bool_value: None,
						string_value: None,
						list_value: None,
						value_type: 0_u8
					},
					ASTNode::Float { .. } => {
						println!("RTE: Cannot apply unary operator `-` to type Float");
						std::process::exit(1);
					},
					ASTNode::Str{..} => {
						println!("RTE: Cannot apply unary operator `-` to type Str");
						std::process::exit(1);
					},
					ASTNode::ExpressionList {..} => {
						println!("RTE: Cannot apply unary operator `-` to type Vector");
						std::process::exit(1);
					},
					_ => {
						println!("(Int) Unary operator `-` not implemented for {operand:?}");
						std::process::exit(1);
					}
				}
			},
			"++" => {
				match *operand {
					ASTNode::ID{ref name} => {						
						let new_value: Option<LazyResult> = match
							self.get_variable_value(name).unwrap()
						{
							LazyResult::Int(val) => {
								Some(LazyResult::Int(val + 1))
							},
							LazyResult::Float(val) => {
								Some(LazyResult::Float(val + 1.0))
							},
							_ => {
								println!("RTE: Wrong use of `++`");
								std::process::exit(1);
							}
						};
						self.current_scope.insert(name.to_string(), new_value.clone());
						return self.lazy2_value(new_value.unwrap());
					},
					_ => {
						println!("RTE: Wrong use of `++`");
						std::process::exit(1);
					}
				}
			},
			"--" => {
				match *operand {
					ASTNode::ID{ref name} => {
						let new_value: Option<LazyResult> = match
							self.get_variable_value(name)
						{
							Some(LazyResult::Int(val)) => {
								Some(LazyResult::Int(val - 1))
							},
							Some(LazyResult::Float(val)) => {
								Some(LazyResult::Float(val - 1.0))
							},
							_ => {
								println!("RTE: Wrong use of `--`");
								std::process::exit(1);
							}
						};
						self.current_scope.insert(name.to_string(), new_value.clone());
						return self.lazy2_value(new_value.unwrap());
					},
					_ => {
						println!("RTE: Wrong use of `--`");
						std::process::exit(1);
					}
				}
			},
			_ => {
				println!("(Int) Unary operator not Implemented {operand:?}");
				std::process::exit(1);
			}
		}
	}

	fn lazy2_value(&mut self, value: LazyResult) -> Value {
		match value {
			LazyResult::List(val) => Value {
				int_value: None,
				float_value: None,
				bool_value: None,
				string_value: None,
				list_value: Some(val.clone()),
				value_type: 5_u8
			},
			LazyResult::Null | LazyResult::None => Value {
				int_value: None,
				float_value: None,
				bool_value: None,
				string_value: None,
				list_value: None,
				value_type: 4_u8
			},
			LazyResult::Str(val) => Value {
				int_value: None,
				float_value: None,
				bool_value: None,
				string_value: Some(val.clone()),
				list_value: None,
				value_type: 3_u8
			},
			LazyResult::Bool(val) => Value {
				int_value: None,
				float_value: None,
				bool_value: Some(val),
				string_value: None,
				list_value: None,
				value_type: 2_u8
			},
			LazyResult::Float(val) => Value {
				int_value: None,
				float_value: Some(val),
				bool_value: None,
				string_value: None,
				list_value: None,
				value_type: 1_u8
			},
			LazyResult::Int(val) => Value {
				int_value: Some(val),
				float_value: None,
				bool_value: None,
				string_value: None,
				list_value: None,
				value_type: 0_u8
			},
			_ => {
				 println!(
				 "RTE: Inconvertible lazy result. \nHint this may be an expresion conversion"
				 );
				 std::process::exit(1);
			 }
		}		
	}
	
	fn value2_lazy(&mut self, value: Value) -> LazyResult {
		 match value.value_type {
			0 => LazyResult::Int(value.int_value.unwrap()),
			1 => LazyResult::Float(value.float_value.unwrap()),
			2 => LazyResult::Bool(value.bool_value.unwrap()),
			3 => LazyResult::Str(value.string_value.clone().unwrap()),
			4 => LazyResult::None,
			5 => LazyResult::List(value.list_value.clone().unwrap()),
			127 => {
				 println!("(Int)Undefined Value Type");
				 std::process::exit(1);
			 },
			_ => {
				 println!("(Int)Invalid Value");
				 std::process::exit(1);
			 }
		}
	}

	fn execute_func(&mut self, func_name: String, args: Vec<ASTNode>) -> LazyResult {
		if self.functions.len() == 0 {
			println!("(Int)Functions are not found.\nIt may be caused by you or me. \nRestart the code(Int)");
			std::process::exit(1);
		}

		let funcs = self.functions.pop().unwrap();
		let (params, block) = match funcs.get(&func_name) {
			Some(val) => val,
			None =>  {
				println!("RTE: Function `{}` not found", &func_name);
				self.functions.push(funcs);
				std::process::exit(1);
			}
		};
		
		//_ -> shows they are yet to be accepted in the program.
		let (_input, _out_param) = params;
		let formal_params: Vec<ASTNode> = match _input {
			Some(p) => p.to_vec(),
			None => vec![]
		};
			
		let p_len = formal_params.len();
			
		if p_len != args.len() {
			let verb  = if args.len() > 1 {	"were" } else { "was" };
			let p = if p_len > 0 { ".." } else { "" };
			
			println!(
				"RTE: Function '{}({p})' expects {} arguments, but {} {verb} provided",
				&func_name, p_len, args.len()
			);
			std::process::exit(1);
		}
		
		let mut new_scope: HashMap<String, Option<LazyResult>> = HashMap::new();
		
		if !args.is_empty() {
			let mut param: &str;
			let mut value: Value;
			let mut lazy_argument: LazyResult;

			for i in 0..args.len() {
				param = match formal_params[i] {
					ASTNode::ID{ref name} => {
						name
					},
					_ => {continue}
				};
				value = self.evaluate(args[i].clone());
				lazy_argument = self.value2_lazy(value);
				new_scope.insert(param.to_string(), Some(lazy_argument));
			}
		}
		self.scopes.push(new_scope.clone());
		self.current_scope = new_scope;

		let func_rn = self.execute_block(block.to_vec());
		//func_rn -> true  = function returned sth
		//           false = function didn't returned anyting

		if func_rn {
			let lazy_rn = self.value2_lazy(self.return_value.clone().unwrap());
			self.return_value = None;
			self.clean_scope();
			return lazy_rn;
		}

		//println!("Executing function: {func_name}...");
		//println!("Scopes: {:?}", self.current_scope);
		
		return LazyResult::Null;
	}

	fn clean_scope(&mut self) {
		// Remove local variables(In the current scope)
		// formal parameter(in current scope)
		// set current scope (top scope of self.scopes)
		self.current_scope = match self.scopes.pop() {
			Some(scope) => scope,
			None => {
				HashMap::new()
			}
		};

		// Remove formal parameter
		self.scopes.pop();
	}

	fn execute_block(&mut self, block: Vec<ASTNode>) -> bool {
		//let mut rn_list: Value = vec![];
		for statement in block {
			// We have ignored that a statement can return a value
			let _ = self.execute_statement(statement);
			if !self.return_value.is_none() {
				return true;
			}
		}
		return false;
	}

	fn func_declaration(
		&mut self,
		name: Rc<ASTNode>,
		parameters: (Option<Vec<ASTNode>>, Option<Vec<ASTNode>>),
		block: Vec<ASTNode>
	) -> LazyResult
	{
		if let Some(mut funcs) = self.functions.pop() {
			let name: String = match *name {
				ASTNode::ID{ref name} => {
					name.to_string()
				},
				_ => {
					println!("Name: {:?}", &name);
					println!("Invalid function name");
					std::process::exit(1);
				}
			};
			funcs.insert(name, (parameters, block));

			self.functions.push(funcs);
		}
		return LazyResult::Null;
	}

	fn var_declaration(&mut self, name: &Rc<ASTNode>, value: Option<Rc<ASTNode>>) -> LazyResult {
		let value = match value {
			Some(value) => {
				match *value {
					ASTNode::Integer{value} => Some(LazyResult::Int(value)),
					ASTNode::Float{value} => Some(LazyResult::Float(value)),
					ASTNode::Bool{value} => Some(LazyResult::Bool(value)),
					ASTNode::Str{ref value} => Some(LazyResult::Str(value.clone())),
					ASTNode::None => Some(LazyResult::None),
					_ => Some(LazyResult::Expression { expr: value.clone()})
				}
			},
			_ => None
		};
		let name: String = match **name {
			ASTNode::ID{ref name} => {
				name.to_string()
			},
			_ => {
				println!("Name: {:?}", &name);
				println!("Invalid variable name");
				std::process::exit(1);
			}
		};
		
		self.current_scope.insert(name, value);

		return LazyResult::Null;
	}
		
}

use std::fs::File;
use std::io::prelude::*;
use std::env::{self, Args};

fn main() {
    let mut args: Args = env::args();
    args.next();

    let file_name = match args.next() {
        Some(c) => c,
        None => {
            println!("Source file not provided");
            std::process::exit(1);
        }
    };
    
    let mut code = String::new();
    let mut f = match File::open(file_name) {
        Ok(c) => c,
        Err(_) => {
            println!("Source file not provided");
            std::process::exit(1);
        }
    };
    
    match f.read_to_string(&mut code) {
        Ok(_) => (),
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    };
    let mut lexer = Lexer::new(code);
    let mut tokens = lexer.lex();

    tokens.reverse();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    //println!("{:?}", ast);
    let mut exec = Executor::new(ast);
    exec.execute();

    //println!("All variables\n");
    //println!("{:#?}", exec.scopes);
    
    //println!("All functions\n");
    //println!("{:#?}", exec.functions);
    
}
