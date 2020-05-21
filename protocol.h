// LCDWRITE {"l2": "string", "l1": "string"}
#define LCDWRITE_VAL_ID 0
typedef struct {
	String l1;
	String l2;
} LcdwriteVal;
typedef void (* OnLcdwriteVal)(LcdwriteVal);

String repr_LcdwriteVal(LcdwriteVal v) {
	String s = "" + char(0);
	s += v.l1;
	s += char(255);
	s += v.l2;
	s += char(255);
	return s;
}
// LED_1 {"state": "bool"}
#define LED_1_VAL_ID 1
typedef struct {
	bool state;
} Led_1Val;
typedef void (* OnLed_1Val)(Led_1Val);

String repr_Led_1Val(Led_1Val v) {
	String s = "" + char(1);
	s += char(0|(int(v.state)<<0));
	return s;
}
#define LED_1_VAL_LEN 2
class Parser {
	public:
		Parser(OnLcdwriteVal, OnLed_1Val);
		void parse(byte);
	private:
		String tokens;
		OnLcdwriteVal onLcdwriteVal;
OnLed_1Val onLed_1Val;
};
Parser::Parser(OnLcdwriteVal tmpOnLcdwriteVal, OnLed_1Val tmpOnLed_1Val) {
	tokens = "";
	onLcdwriteVal = tmpOnLcdwriteVal;
onLed_1Val = tmpOnLed_1Val;
};
void Parser::parse(byte b) {
	if(tokens.length() == 0) {
		switch (b) {
			case LCDWRITE_VAL_ID:
tokens += char(b);break;
case LED_1_VAL_ID:
tokens += char(b);break;
			default:
				break;
		}
	} else {
		tokens += char(b);
		switch(int(tokens.charAt(0))) {
			case LCDWRITE_VAL_ID:{
bool done = false;
int byte_count = 1;
while(tokens.charAt(byte_count) != char(255)) {
					byte_count++;
					if (byte_count >= tokens.length()) { return; }
				}
				byte_count++;
while(tokens.charAt(byte_count) != char(255)) {
					byte_count++;
					if (byte_count >= tokens.length()) { return; }
				}
				byte_count++;
done = tokens.length() == byte_count;
if(done) {
int last_index = 1;
String l1 = "";
while (tokens[last_index] != char(255)) {
l1 += char(tokens[last_index]);
last_index ++;
}
last_index ++;
String l2 = "";
while (tokens[last_index] != char(255)) {
l2 += char(tokens[last_index]);
last_index ++;
}
last_index ++;
LcdwriteVal ret;
ret.l2 = l2;
ret.l1 = l1;onLcdwriteVal(ret);tokens="";
}
break;}

case LED_1_VAL_ID:{
bool done = false;
done = (tokens.length() == LED_1_VAL_LEN);
if(done) {
int last_index = 1;
bool state = (tokens[last_index] & (1 << 0)) != 0;
last_index += 1;
Led_1Val ret;
ret.state = state;onLed_1Val(ret);tokens="";
}
break;}

			default:
				break;
		}
	}
};
