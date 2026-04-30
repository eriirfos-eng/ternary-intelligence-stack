import ast
from dataclasses import dataclass
from typing import List, Any

@dataclass
class SSAInstruction:
    opcode: str
    operands: List[Any]
    result: str

class PyTernSSABuilder(ast.NodeVisitor):
    """
    Builds SSA-based IR for high-performance optimization.
    """
    def __init__(self):
        self.instructions: List[SSAInstruction] = []
        self._temp_count = 0

    def _new_temp(self) -> str:
        self._temp_count += 1
        return f"%t{self._temp_count}"

    def visit_BinOp(self, node):
        self.generic_visit(node)
        left = node.left.id if isinstance(node.left, ast.Name) else "const"
        right = node.right.id if isinstance(node.right, ast.Name) else "const"
        
        op = "TV_ADD" if isinstance(node.op, ast.Add) else "TV_MUL"
        result = self._new_temp()
        self.instructions.append(SSAInstruction(op, [left, right], result))

    def visit_Call(self, node):
        self.generic_visit(node)
        if isinstance(node.func, ast.Name):
            if node.func.id == 'sync_swarm':
                self.instructions.append(SSAInstruction("TSYNC", [], "%sync"))
            elif node.func.id == 'consensus':
                self.instructions.append(SSAInstruction("TCONSENSUS", [arg.id for arg in node.args], "%consensus"))

    def get_ir(self) -> List[SSAInstruction]:
        return self.instructions
