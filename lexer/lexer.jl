@enum TokenKind begin
    Ident
    LiteralInt
    LiteralFloat
end

struct Lexem
    kind::TokenKind
    i_start::Int
    i_end::Int
end

mutable struct Lexer
    input::String
    pos::UInt
end

function match_token_kind(text::String)::TokenKind
    if isa(tryparse(Int, text), Int)
        return LiteralInt
    elseif isa(tryparse(Float64, text), Float64)
        return LiteralFloat
    end
    return Ident
end

function next_token(lex::Lexer)::Union{Lexem, Nothing}
    input_len = length(lex.input)

    if lex.pos > input_len
        return nothing
    end

    tkn_start = lex.pos
    #println(lex.pos)

    while !isspace(lex.input[lex.pos]) && lex.pos < input_len
        lex.pos += 1
    end

    #println(lex.pos)
    tkn_end = if lex.pos == input_len; lex.pos else lex.pos - 1 end
    lex.pos += 1

    return Lexem(match_token_kind(lex.input[tkn_start:tkn_end]), tkn_start, tkn_end)
end

# 30 => 1:2
# 1º space => 3
# abc => 4:6
# 2º space => 7
# 45.7 => 8:11
# 3º space => 12
# def => 13:15
t::Lexer = Lexer("30 abc 45.7 def", 1)
println(next_token(t))
println(next_token(t))
println(next_token(t))
println(next_token(t))
