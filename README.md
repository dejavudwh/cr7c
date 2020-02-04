# cr7c

![](https://img.shields.io/badge/language-Rust-brown.svg)
![](https://img.shields.io/badge/category-learning-blue.svg)
[![](https://img.shields.io/badge/blog-@dejavudwh-red.svg)](https://dejavudwh.cn/)
![](https://img.shields.io/badge/progress-doing-green.svg)


> Cr (c-rust) compiler, a simplified C compiler to X86 written in Rust

*This project is just for me to learn Rust. Doesn't make much sense, bad code style*

## TODO LIST

- ### [√] Lexical analysis  
- ### [√] Syntax analysis  
- ### [√] Rough semantic analysis
- ### [ ] Intermediate Representation
- ### [x] Code generation  

## Example

### Source code

```c
struct student {
    char[] name;
    int age;
    int gender;
    struct teacher *t;
};

struct teacher {
    struct student[] s;
    struct headmaster *hm;
};

struct headmaster {
    struct teacher[] s;
};

int main(int argc,char **argv) {
    struct student a;
    int ba1 = 1;
    int[] aa;
    int[10] bb = aa;
    bb[ba1] = 1 * 2 << 3 && 4 + 5 / 6 + calc(2);
    a.age = 2 + calc(a);
    &a;
    for(i = 0; i < 10; i++) {
        int ca = 1;
        if (a == 2) {
            int da = 12;
            break;
        } else {
            continue;
        }
    }

    while(1 == 2) {
        int ca = 1;
        ca = 2;
    }

    return 0;
}

int calc(int a) {
    
}
```

### Abstract syntax tree

```js
ProgramNode {
    import_stmts: [],
    defs: TopDefNode {
        var_defs: [
        DefStructNode {
            name: "student",
            member_list: [SlotNode {
                typeref: TypeNode {
                    type_base: TypeBase {
                        base: Char,
                        name: None
                    },
                    nested_def: [Array]
                },
                name: "name"
            },
            SlotNode {
                typeref: TypeNode {
                    type_base: TypeBase {
                        base: Int,
                        name: None
                    },
                    nested_def: []
                },
                name: "age"
            },
            SlotNode {
                typeref: TypeNode {
                    type_base: TypeBase {
                        base: Int,
                        name: None
                    },
                    nested_def: []
                },
                name: "gender"
            },
            SlotNode {
                typeref: TypeNode {
                    type_base: TypeBase {
                        base: Struct,
                        name: Some("teacher")
                    },
                    nested_def: [Pointer]
                },
                name: "t"
            }]
        },
        DefStructNode {
            name: "teacher",
            member_list: [SlotNode {
                typeref: TypeNode {
                    type_base: TypeBase {
                        base: Struct,
                        name: Some("student")
                    },
                    nested_def: [Array]
                },
                name: "s"
            },
            SlotNode {
                typeref: TypeNode {
                    type_base: TypeBase {
                        base: Struct,
                        name: Some("headmaster")
                    },
                    nested_def: [Pointer]
                },
                name: "hm"
            }]
        },
        DefStructNode {
            name: "headmaster",
            member_list: [SlotNode {
                typeref: TypeNode {
                    type_base: TypeBase {
                        base: Struct,
                        name: Some("teacher")
                    },
                    nested_def: [Array]
                },
                name: "s"
            }]
        }],
        func_defs: [DefFuncNode {
            typeref: TypeNode {
                type_base: TypeBase {
                    base: Int,
                    name: None
                },
                nested_def: []
            },
            name: "main",
            params: ParamsNode {
                params: [SlotNode {
                    typeref: TypeNode {
                        type_base: TypeBase {
                            base: Int,
                            name: None
                        },
                        nested_def: []
                    },
                    name: "argc"
                },
                SlotNode {
                    typeref: TypeNode {
                        type_base: TypeBase {
                            base: Char,
                            name: None
                        },
                        nested_def: [Pointer, Pointer]
                    },
                    name: "argv"
                }]
            },
            block: BlockNode {
                defvars: [
                DefVarNode {
                    typeref: TypeNode {
                        type_base: TypeBase {
                            base: Struct,
                            name: Some("student")
                        },
                        nested_def: []
                    },
                    name_map: {
                        "a": None
                    }
                },
                DefVarNode {
                    typeref: TypeNode {
                        type_base: TypeBase {
                            base: Int,
                            name: None
                        },
                        nested_def: []
                    },
                    name_map: {
                        "ba1": Some(TermNode {
                            case_type: None,
                            unary: SingeUnaryNode {
                                prefix: None,
                                primary: PrimaryNode {
                                    name: None,
                                    value: Integer(1)
                                }
                            }
                        })
                    }
                },
                DefVarNode {
                    typeref: TypeNode {
                        type_base: TypeBase {
                            base: Int,
                            name: None
                        },
                        nested_def: [Array]
                    },
                    name_map: {
                        "aa": None
                    }
                },
                DefVarNode {
                    typeref: TypeNode {
                        type_base: TypeBase {
                            base: Int,
                            name: None
                        },
                        nested_def: [FixedArray(10)]
                    },
                    name_map: {
                        "bb": Some(TermNode {
                            case_type: None,
                            unary: SingeUnaryNode {
                                prefix: None,
                                primary: PrimaryNode {
                                    name: Some("aa"),
                                    value: Identifier
                                }
                            }
                        })
                    }
                }],
                stmts: [ExprStmtNode {
                    expr: AssginmentNode {
                        left_value: TermNode {
                            case_type: None,
                            unary: ArrayUnaryNode {
                                prefix: None,
                                primary: PrimaryNode {
                                    name: Some("bb"),
                                    value: Identifier
                                },
                                postfix: [TermNode {
                                    case_type: None,
                                    unary: SingeUnaryNode {
                                        prefix: None,
                                        primary: PrimaryNode {
                                            name: Some("ba1"),
                                            value: Identifier
                                        }
                                    }
                                }]
                            }
                        },
                        right_value: ArithmeticOpNode {
                            operator: And,
                            left_value: ArithmeticOpNode {
                                operator: Leftshift,
                                left_value: ArithmeticOpNode {
                                    operator: Mul,
                                    left_value: TermNode {
                                        case_type: None,
                                        unary: SingeUnaryNode {
                                            prefix: None,
                                            primary: PrimaryNode {
                                                name: None,
                                                value: Integer(1)
                                            }
                                        }
                                    },
                                    right_value: TermNode {
                                        case_type: None,
                                        unary: SingeUnaryNode {
                                            prefix: None,
                                            primary: PrimaryNode {
                                                name: None,
                                                value: Integer(2)
                                            }
                                        }
                                    }
                                },
                                right_value: TermNode {
                                    case_type: None,
                                    unary: SingeUnaryNode {
                                        prefix: None,
                                        primary: PrimaryNode {
                                            name: None,
                                            value: Integer(3)
                                        }
                                    }
                                }
                            },
                            right_value: ArithmeticOpNode {
                                operator: Add,
                                left_value: ArithmeticOpNode {
                                    operator: Add,
                                    left_value: TermNode {
                                        case_type: None,
                                        unary: SingeUnaryNode {
                                            prefix: None,
                                            primary: PrimaryNode {
                                                name: None,
                                                value: Integer(4)
                                            }
                                        }
                                    },
                                    right_value: ArithmeticOpNode {
                                        operator: Div,
                                        left_value: TermNode {
                                            case_type: None,
                                            unary: SingeUnaryNode {
                                                prefix: None,
                                                primary: PrimaryNode {
                                                    name: None,
                                                    value: Integer(5)
                                                }
                                            }
                                        },
                                        right_value: TermNode {
                                            case_type: None,
                                            unary: SingeUnaryNode {
                                                prefix: None,
                                                primary: PrimaryNode {
                                                    name: None,
                                                    value: Integer(6)
                                                }
                                            }
                                        }
                                    }
                                },
                                right_value: TermNode {
                                    case_type: None,
                                    unary: FuncCallNode {
                                        prefix: None,
                                        primary: PrimaryNode {
                                            name: Some("calc"),
                                            value: Identifier
                                        },
                                        params: Some([TermNode {
                                            case_type: None,
                                            unary: SingeUnaryNode {
                                                prefix: None,
                                                primary: PrimaryNode {
                                                    name: None,
                                                    value: Integer(2)
                                                }
                                            }
                                        }])
                                    }
                                }
                            }
                        }
                    }
                },
                ExprStmtNode {
                    expr: AssginmentNode {
                        left_value: TermNode {
                            case_type: None,
                            unary: RefUnaryNode {
                                prefix: None,
                                operator: Dot,
                                primary: PrimaryNode {
                                    name: Some("a"),
                                    value: Identifier
                                },
                                postfix: Some(SingeUnaryNode {
                                    prefix: None,
                                    primary: PrimaryNode {
                                        name: Some("age"),
                                        value: Identifier
                                    }
                                })
                            }
                        },
                        right_value: ArithmeticOpNode {
                            operator: Add,
                            left_value: TermNode {
                                case_type: None,
                                unary: SingeUnaryNode {
                                    prefix: None,
                                    primary: PrimaryNode {
                                        name: None,
                                        value: Integer(2)
                                    }
                                }
                            },
                            right_value: TermNode {
                                case_type: None,
                                unary: FuncCallNode {
                                    prefix: None,
                                    primary: PrimaryNode {
                                        name: Some("calc"),
                                        value: Identifier
                                    },
                                    params: Some([TermNode {
                                        case_type: None,
                                        unary: SingeUnaryNode {
                                            prefix: None,
                                            primary: PrimaryNode {
                                                name: Some("a"),
                                                value: Identifier
                                            }
                                        }
                                    }])
                                }
                            }
                        }
                    }
                },
                ExprStmtNode {
                    expr: TermNode {
                        case_type: None,
                        unary: SingeUnaryNode {
                            prefix: Some(Bitand),
                            primary: PrimaryNode {
                                name: Some("a"),
                                value: Identifier
                            }
                        }
                    }
                },
                ForStmtNode {
                    initial_expr: AssginmentNode {
                        left_value: TermNode {
                            case_type: None,
                            unary: SingeUnaryNode {
                                prefix: None,
                                primary: PrimaryNode {
                                    name: Some("i"),
                                    value: Identifier
                                }
                            }
                        },
                        right_value: TermNode {
                            case_type: None,
                            unary: SingeUnaryNode {
                                prefix: None,
                                primary: PrimaryNode {
                                    name: None,
                                    value: Integer(0)
                                }
                            }
                        }
                    },
                    condition: ArithmeticOpNode {
                        operator: Less,
                        left_value: TermNode {
                            case_type: None,
                            unary: SingeUnaryNode {
                                prefix: None,
                                primary: PrimaryNode {
                                    name: Some("i"),
                                    value: Identifier
                                }
                            }
                        },
                        right_value: TermNode {
                            case_type: None,
                            unary: SingeUnaryNode {
                                prefix: None,
                                primary: PrimaryNode {
                                    name: None,
                                    value: Integer(10)
                                }
                            }
                        }
                    },
                    end_expr: TermNode {
                        case_type: None,
                        unary: SelfOpUnaryNode {
                            prefix: None,
                            primary: PrimaryNode {
                                name: Some("i"),
                                value: Identifier
                            },
                            postfix: Inc
                        }
                    },
                    stmts: BlockNode {
                        defvars: [DefVarNode {
                            typeref: TypeNode {
                                type_base: TypeBase {
                                    base: Int,
                                    name: None
                                },
                                nested_def: []
                            },
                            name_map: {
                                "ca": Some(TermNode {
                                    case_type: None,
                                    unary: SingeUnaryNode {
                                        prefix: None,
                                        primary: PrimaryNode {
                                            name: None,
                                            value: Integer(1)
                                        }
                                    }
                                })
                            }
                        }],
                        stmts: [IfStmtNode {
                            condition: ArithmeticOpNode {
                                operator: Equal,
                                left_value: TermNode {
                                    case_type: None,
                                    unary: SingeUnaryNode {
                                        prefix: None,
                                        primary: PrimaryNode {
                                            name: Some("a"),
                                            value: Identifier
                                        }
                                    }
                                },
                                right_value: TermNode {
                                    case_type: None,
                                    unary: SingeUnaryNode {
                                        prefix: None,
                                        primary: PrimaryNode {
                                            name: None,
                                            value: Integer(2)
                                        }
                                    }
                                }
                            },
                            if_stmt: BlockNode {
                                defvars: [DefVarNode {
                                    typeref: TypeNode {
                                        type_base: TypeBase {
                                            base: Int,
                                            name: None
                                        },
                                        nested_def: []
                                    },
                                    name_map: {
                                        "da": Some(TermNode {
                                            case_type: None,
                                            unary: SingeUnaryNode {
                                                prefix: None,
                                                primary: PrimaryNode {
                                                    name: None,
                                                    value: Integer(12)
                                                }
                                            }
                                        })
                                    }
                                }],
                                stmts: [BreakStmtNode]
                            },
                            else_stmt: Some(BlockNode {
                                defvars: [],
                                stmts: [ContinueStmtNode]
                            })
                        }]
                    }
                },
                WhileStmtNode {
                    condition: ArithmeticOpNode {
                        operator: Equal,
                        left_value: TermNode {
                            case_type: None,
                            unary: SingeUnaryNode {
                                prefix: None,
                                primary: PrimaryNode {
                                    name: None,
                                    value: Integer(1)
                                }
                            }
                        },
                        right_value: TermNode {
                            case_type: None,
                            unary: SingeUnaryNode {
                                prefix: None,
                                primary: PrimaryNode {
                                    name: None,
                                    value: Integer(2)
                                }
                            }
                        }
                    },
                    stmts: BlockNode {
                        defvars: [DefVarNode {
                            typeref: TypeNode {
                                type_base: TypeBase {
                                    base: Int,
                                    name: None
                                },
                                nested_def: []
                            },
                            name_map: {
                                "ca": Some(TermNode {
                                    case_type: None,
                                    unary: SingeUnaryNode {
                                        prefix: None,
                                        primary: PrimaryNode {
                                            name: None,
                                            value: Integer(1)
                                        }
                                    }
                                })
                            }
                        }],
                        stmts: [ExprStmtNode {
                            expr: AssginmentNode {
                                left_value: TermNode {
                                    case_type: None,
                                    unary: SingeUnaryNode {
                                        prefix: None,
                                        primary: PrimaryNode {
                                            name: Some("ca"),
                                            value: Identifier
                                        }
                                    }
                                },
                                right_value: TermNode {
                                    case_type: None,
                                    unary: SingeUnaryNode {
                                        prefix: None,
                                        primary: PrimaryNode {
                                            name: None,
                                            value: Integer(2)
                                        }
                                    }
                                }
                            }
                        }]
                    }
                },
                ReturnStmtNode {
                    value: TermNode {
                        case_type: None,
                        unary: SingeUnaryNode {
                            prefix: None,
                            primary: PrimaryNode {
                                name: None,
                                value: Integer(0)
                            }
                        }
                    }
                }]
            }
        },
        DefFuncNode {
            typeref: TypeNode {
                type_base: TypeBase {
                    base: Int,
                    name: None
                },
                nested_def: []
            },
            name: "calc",
            params: ParamsNode {
                params: [SlotNode {
                    typeref: TypeNode {
                        type_base: TypeBase {
                            base: Int,
                            name: None
                        },
                        nested_def: []
                    },
                    name: "a"
                }]
            },
            block: BlockNode {
                defvars: [],
                stmts: []
            }
        }]
    }
}
```

### Target code(GNU as)