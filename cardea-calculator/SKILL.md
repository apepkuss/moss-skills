---
name: cardea-calculator
description: |
  Invoke Cardea Calculator MCP Server for mathematical calculations.
  Supports addition (sum) and subtraction (sub) operations.
  Use this skill when users need simple math calculations or explicitly request to use cardea calculator.
allowed-tools: mcp__cardea-calculator__sum, mcp__cardea-calculator__sub
---

# Cardea Calculator MCP Skill

Perform mathematical operations by calling the Cardea Calculator service via MCP protocol.

## Available Tools

This skill invokes the following MCP tools:

| Tool Name | Function | Parameters |
|-----------|----------|------------|
| `mcp__cardea-calculator__sum` | Calculate the sum of two numbers | `a`: integer, `b`: integer |
| `mcp__cardea-calculator__sub` | Calculate the difference of two numbers | `a`: integer, `b`: integer |

## Usage Instructions

When users request mathematical calculations:

### Addition

Use the `mcp__cardea-calculator__sum` tool:

```
Tool: mcp__cardea-calculator__sum
Parameters:
  a: <first integer>
  b: <second integer>
```

**Example**: Calculate 15 + 27
```
Call mcp__cardea-calculator__sum with a=15, b=27
Result: 42
```

### Subtraction

Use the `mcp__cardea-calculator__sub` tool:

```
Tool: mcp__cardea-calculator__sub
Parameters:
  a: <minuend>
  b: <subtrahend>
```

**Example**: Calculate 100 - 37
```
Call mcp__cardea-calculator__sub with a=100, b=37
Result: 63
```

## Parameter Reference

| Parameter | Type | Description |
|-----------|------|-------------|
| `a` | i32 (32-bit integer) | Left operand |
| `b` | i32 (32-bit integer) | Right operand |

**Note**: Parameters must be integers. Floating-point numbers are not supported.

## Execution Flow

1. Parse the user's mathematical expression
2. Identify the operation type (addition or subtraction)
3. Extract the operands
4. Call the corresponding MCP tool
5. Return the calculation result

## Compound Operations

For expressions containing multiple operations, decompose and call sequentially:

**Example**: Calculate 10 + 5 - 3

1. First call `sum(a=10, b=5)` to get 15
2. Then call `sub(a=15, b=3)` to get 12

## Error Handling

- If the MCP service is not connected, prompt the user to check the service status
- If parameters are not integers, prompt the user to enter valid integers
- If values exceed the i32 range (-2147483648 to 2147483647), notify the user

## Service Information

| Property | Value |
|----------|-------|
| Service Name | cardea-calculator |
| Version | 0.1.0 |
| Default Port | 8001 |
| Transport Protocol | Streamable HTTP |
| Endpoint | /mcp |

## Usage Examples

Users can make requests like:

- "Use cardea to calculate 123 + 456"
- "Help me calculate 1000 - 250"
- "Calculate 50 plus 30"
- "What is 100 minus 45"
