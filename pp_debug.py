import sys
import os

def pprint_debug_output(text):
    INDENT_SPACES = 3
    indent = 0
    start = 0
    MAX_DIST = 40
    i = 0
    after_newline = False
    while i < len(text):
        ch = text[i]
        if ch == "{":
            print("{}{}".format(indent * " ", text[start : i+1].strip()))
            after_newline = True
            indent += INDENT_SPACES
            i += 1
            start = i
        
        elif ch == ",":
            print("{}{}".format(indent * " ", text[start : i+1].strip()))
            after_newline = True
            i += 1
            start = i
        
        # Same as the below case, should probably be refactored
        elif ch == "(":
            j = text.find(")", i)
            k = text.find("(", i + 1)
            dist = j - i
            if j == -1:
                i += 1
                after_newline = False
            
            elif (k != -1 and k < j) or dist > MAX_DIST:
                print("{}{}".format(indent * " ", text[start : i+1].strip()))
                after_newline = True
                indent += INDENT_SPACES
                i += 1
                start = i

            elif dist < 40:
                after_newline = False
                #print("{}".format(text[start : j+1].strip()), end="")
                i = j + 1
                #start = i
                
            else:
                raise Exception("Unreachable!")
        
        elif ch == "[":
            j = text.find("]", i)
            k = text.find("[", i + 1)
            dist = j - i
            if j == -1:
                i += 1
                after_newline = False
            
            elif (k != -1 and k < j) or dist > MAX_DIST:
                print("{}{}".format(indent * " ", text[start : i+1].strip()))
                after_newline = True
                indent += INDENT_SPACES
                i += 1
                start = i

            elif dist < 40:
                after_newline = False
                #print("{}".format(text[start : j+1].strip()), end="")
                i = j + 1
                #start = i
                
            else:
                raise Exception("Unreachable!")
        
        elif ch == "}" or ch == "]" or ch == ")":
            end = "" if after_newline else os.linesep
            part = text[start : i].strip()
            if part != "":
                print("{}{}".format(indent * " ", part), end=end)
            indent -= INDENT_SPACES
            print("{}{}".format(indent * " ", ch))
            after_newline = True
            i += 1
            start = i
        
        else:
            if not ch.isspace():
                after_newline = False
            i += 1
            
    if start != len(text):
        print(text[start:])

def main(args=sys.argv[1:]):
    """docstring for main"""
    if len(args) > 1:
        return print("Usage: python3 pp_debug.py [text | stdin by default]")
    
    if len(args) == 0:
        lines = []
        try:
            while True:
                lines.append(input())
        except EOFError:
            pass
        text = os.linesep.join(lines)
        #print("Text: {}".format(text))
    else:
        text = args[0]
    pprint_debug_output(text)


if __name__ == '__main__':
    main()
