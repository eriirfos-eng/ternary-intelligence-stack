from ssa_builder import PyTernSSABuilder
import ast

def test_ssa_generation():
    source = "x = a + b"
    tree = ast.parse(source)
    builder = PyTernSSABuilder()
    builder.visit(tree)
    ir = builder.get_ir()
    
    print(f"Generated SSA IR: {ir}")
    assert len(ir) == 1
    assert ir[0].opcode == "TV_ADD"
    assert ir[0].result == "%t1"
    print("Test Passed: SSA Generation")

if __name__ == "__main__":
    test_ssa_generation()
