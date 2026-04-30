import ast

class PyTernASTVisitor(ast.NodeVisitor):
    """
    AST Visitor to map constrained Python operations to Ternary IR.
    """
    def __init__(self):
        self.ir_instructions = []

    def visit_FunctionDef(self, node):
        self.ir_instructions.append(f"FUNC_START: {node.name}")
        self.generic_visit(node)
        self.ir_instructions.append("FUNC_END")

    def visit_BinOp(self, node):
        if isinstance(node.op, ast.Add):
            self.ir_instructions.append("TV_ADD")
        elif isinstance(node.op, ast.Mult):
            self.ir_instructions.append("TV_MUL")
        self.generic_visit(node)

    def visit_Call(self, node):
        if isinstance(node.func, ast.Name) and node.func.id == 't_relu':
            self.ir_instructions.append("TV_TRELU")
        self.generic_visit(node)

    def get_ir(self):
        return "\n".join(self.ir_instructions)

def transpile(source_code: str):
    tree = ast.parse(source_code)
    visitor = PyTernASTVisitor()
    visitor.visit(tree)
    return visitor.get_ir()
