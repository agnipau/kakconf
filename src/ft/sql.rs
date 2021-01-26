use std::fmt::Write;

pub const KEYWORDS: [&str; 47] = [
    "ALTER",
    "AS",
    "ASC",
    "AUTO_INCREMENT",
    "CHECK",
    "CONSTRAINT",
    "CREATE",
    "DATABASE",
    "DEFAULT",
    "DELETE",
    "DESC",
    "DISTINCT",
    "DROP",
    "EXISTS",
    "FOREIGN KEY",
    "FROM",
    "FULL JOIN",
    "FULL OUTER JOIN",
    "GROUP BY",
    "HAVING",
    "INDEX",
    "INNER JOIN",
    "INSERT INTO",
    "INTO",
    "JOIN",
    "LEFT JOIN",
    "LEFT OUTER JOIN",
    "LIMIT",
    "MODIFY",
    "NOT NULL",
    "ON",
    "ORDER BY",
    "PRIMARY KEY",
    "REFERENCES",
    "RIGHT JOIN",
    "RIGHT OUTER JOIN",
    "SELECT",
    "SELECT TOP",
    "SET",
    "TABLE",
    "TRUNCATE",
    "UNION",
    "UNIQUE",
    "UPDATE",
    "VALUES",
    "VIEW",
    "WHERE",
];

pub const OPERATORS: [&str; 11] = [
    "ALL", "AND", "ANY", "BETWEEN", "EXISTS", "IN", "IS", "LIKE", "NOT", "OR", "SOME",
];

pub const MYSQL_FUNCTIONS: [&str; 135] = [
    "ABS",
    "ACOS",
    "ADDDATE",
    "ADDTIME",
    "ASCII",
    "ASIN",
    "ATAN",
    "AVG",
    "BIN",
    "BINARY",
    "CASE",
    "CAST",
    "CEIL",
    "CEILING",
    "CHARACTER_LENGTH",
    "CHAR_LENGTH",
    "COALESCE",
    "CONCAT",
    "CONCAT_WS",
    "CONNECTION_ID",
    "CONV",
    "CONVERT",
    "COS",
    "COT",
    "COUNT",
    "CURDATE",
    "CURRENT_DATE",
    "CURRENT_TIME",
    "CURRENT_TIMESTAMP",
    "CURRENT_USER",
    "CURTIME",
    "DATABASE",
    "DATE",
    "DATE_ADD",
    "DATEDIFF",
    "DATE_FORMAT",
    "DATE_SUB",
    "DAY",
    "DAYNAME",
    "DAYOFMONTH",
    "DAYOFWEEK",
    "DAYOFYEAR",
    "DEGREES",
    "DIV",
    "EXP",
    "EXTRACT",
    "FIELD",
    "FIND_IN_SET",
    "FLOOR",
    "FORMAT",
    "FROM_DAYS",
    "GREATEST",
    "HOUR",
    "IF",
    "IFNULL",
    "INSERT",
    "INSTR",
    "ISNULL",
    "LAST_DAY",
    "LAST_INSERT_ID",
    "LCASE",
    "LEAST",
    "LEFT",
    "LENGTH",
    "LN",
    "LOCALTIME",
    "LOCALTIMESTAMP",
    "LOCATE",
    "LOG",
    "LOWER",
    "LPAD",
    "LTRIM",
    "MAKEDATE",
    "MAKETIME",
    "MAX",
    "MICROSECOND",
    "MID",
    "MIN",
    "MINUTE",
    "MOD",
    "MONTH",
    "MONTHNAME",
    "NOW",
    "NULLIF",
    "PERIOD_ADD",
    "PERIOD_DIFF",
    "PI",
    "POSITION",
    "POW",
    "POWER",
    "QUARTER",
    "RADIANS",
    "RAND",
    "REPEAT",
    "REPLACE",
    "REVERSE",
    "RIGHT",
    "ROUND",
    "RPAD",
    "RTRIM",
    "SECOND",
    "SEC_TO_TIME",
    "SESSION_USER",
    "SIGN",
    "SIN",
    "SPACE",
    "SQRT",
    "STRCMP",
    "STR_TO_DATE",
    "SUBDATE",
    "SUBSTR",
    "SUBSTRING",
    "SUBSTRING_INDEX",
    "SUBTIME",
    "SUM",
    "SYSDATE",
    "SYSTEM_USER",
    "TAN",
    "TIME",
    "TIMEDIFF",
    "TIME_FORMAT",
    "TIMESTAMP",
    "TIME_TO_SEC",
    "TO_DAYS",
    "TRIM",
    "TRUNCATE",
    "UCASE",
    "UPPER",
    "USER",
    "VERSION",
    "WEEK",
    "WEEKDAY",
    "WEEKOFYEAR",
    "YEAR",
    "YEARWEEK",
];

pub const SQL_SERVER_FUNCTIONS: [&str; 17] = [
    "CHAR",
    "CHARINDEX",
    "DATALENGTH",
    "DATEADD",
    "DATENAME",
    "DATEPART",
    "GETDATE",
    "GETUTCDATE",
    "ISDATE",
    "ISNUMERIC",
    "LEN",
    "NCHAR",
    "PATINDEX",
    "SESSIONPROPERTY",
    "STR",
    "STUFF",
    "USER_NAME",
];

pub const MS_ACCESS_FUNCTIONS: [&str; 63] = [
    "Abs",
    "Asc",
    "Atn",
    "Avg",
    "Chr",
    "Cos",
    "Count",
    "CurDir",
    "CurrentUser",
    "Date",
    "DateAdd",
    "DateDiff",
    "DatePart",
    "DateSerial",
    "DateValue",
    "Day",
    "Environ",
    "Exp",
    "Fix",
    "Format",
    "Hour",
    "InStr",
    "InstrRev",
    "Int",
    "IsDate",
    "IsNull",
    "IsNumeric",
    "LCase",
    "Left",
    "Len",
    "LTrim",
    "Max",
    "Mid",
    "Min",
    "Minute",
    "Month",
    "MonthName",
    "Now",
    "Randomize",
    "Replace",
    "Right",
    "Rnd",
    "Round",
    "RTrim",
    "Second",
    "Sgn",
    "Space",
    "Split",
    "Sqr",
    "Str",
    "StrComp",
    "StrConv",
    "StrReverse",
    "Sum",
    "Time",
    "TimeSerial",
    "TimeValue",
    "Trim",
    "UCase",
    "Val",
    "Weekday",
    "WeekdayName",
    "Year",
];

pub const ORACLE_FUNCTIONS: [&str; 33] = [
    "ADD_MONTHS",
    "ASCIISTR",
    "BITAND",
    "CHR",
    "COMPOSE",
    "COSH",
    "DBTIMEZONE",
    "DECOMPOSE",
    "DUMP",
    "INITCAP",
    "INSTRB",
    "INSTRC",
    "LENGTHB",
    "LENGTHC",
    "MEDIAN",
    "MONTHS_BETWEEN",
    "NCHR",
    "NEW_TIME",
    "NEXT_DAY",
    "REGEXP_COUNT",
    "REGEXP_INSTR",
    "REGEXP_REPLACE",
    "REGEXP_SUBSTR",
    "REMAINDER",
    "ROWNUM",
    "SESSIONTIMEZONE",
    "SOUNDEX",
    "SYSTIMESTAMP",
    "TANH",
    "TRANSLATE",
    "TRUNC",
    "TZ_OFFSET",
    "VSIZE",
];

pub const MYSQL_DATA_TYPES: [&str; 7] = [
    "LONGBLOB",
    "LONGTEXT",
    "MEDIUMBLOB",
    "MEDIUMTEXT",
    "SET",
    "TEXT",
    "TINYTEXT",
];

pub const MYSQL_DATA_TYPES_FN: [&str; 17] = [
    "BIGINT",
    "BLOB",
    "CHAR",
    "DATE",
    "DATETIME",
    "DECIMAL",
    "DOUBLE",
    "ENUM",
    "FLOAT",
    "INT",
    "MEDIUMINT",
    "SMALLINT",
    "TIME",
    "TIMESTAMP",
    "TINYINT",
    "VARCHAR",
    "YEAR",
];

pub const SQL_SERVER_DATA_TYPES: [&str; 26] = [
    "bigint",
    "bit",
    "cursor",
    "date",
    "datetime",
    "datetime2",
    "datetimeoffset",
    "image",
    "int",
    "money",
    "nchar",
    "ntext",
    "nvarchar",
    "real",
    "smalldatetime",
    "smallint",
    "smallmoney",
    "sql_variant",
    "table",
    "text",
    "time",
    "timestamp",
    "tinyint",
    "uniqueidentifier",
    "varbinary",
    "xml",
];

pub const SQL_SERVER_DATA_TYPES_FN: [&str; 9] = [
    "binary",
    "char",
    "decimal",
    "float",
    "numeric",
    "nvarchar",
    "varbinary",
    "varchar",
    "varchar",
];

pub const MS_ACCESS_DATA_TYPES: [&str; 14] = [
    "Text",
    "Memo",
    "Byte",
    "Integer",
    "Long",
    "Single",
    "Double",
    "Currency",
    "AutoNumber",
    "Date",
    "Time",
    "Ole Object",
    "Hyperlink",
    "Lookup Wizard",
];

pub fn sql() -> anyhow::Result<String> {
    let mut buf = String::new();

    write!(
        buf,
        r#"
# Detection.
# ‾‾‾‾‾‾‾‾‾‾

hook global BufCreate .*/?(?i)sql %[
    set-option buffer filetype sql
]

# Initialization.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾

hook global WinSetOption filetype=sql %[
    require-module sql
    set-option window static_words %opt[sql_static_words]
]

hook -group sql-highlight global WinSetOption filetype=sql %[
    add-highlighter window/sql ref sql
    hook -once -always window WinSetOption filetype=.* %[ remove-highlighter window/sql ]
]

provide-module sql %[

# Highlighters.
# ‾‾‾‾‾‾‾‾‾‾‾‾‾

add-highlighter shared/sql regions
add-highlighter shared/sql/code default-region group
add-highlighter shared/sql/double_string region '"' (?<!\\)(\\\\)*" fill string
add-highlighter shared/sql/single_string region "'" (?<!\\)(\\\\)*' fill string
add-highlighter shared/sql/comment1 region '--' '$' fill comment
add-highlighter shared/sql/comment2 region '#' '$' fill comment
add-highlighter shared/sql/comment3 region '/\*' '\*/' fill comment

# Add the language's grammar to the static completion list.
declare-option str-list sql_static_words {keywords_all} NULL

# Highlight keywords.
add-highlighter shared/sql/code/ regex '(?i)\b({functions})\(.*?\)' 0:function
add-highlighter shared/sql/code/ regex '(?i)\b({data_types_fn})\(.*?\)' 0:type
add-highlighter shared/sql/code/ regex '(?i)\b({keywords})\b' 0:keyword
add-highlighter shared/sql/code/ regex '(?i)\b({operators})\b' 0:operator
add-highlighter shared/sql/code/ regex '(?i)\b({data_types})\b' 0:type

add-highlighter shared/sql/code/ regex '\+|-|\*|/|%|&|\||^|=|>|<|>=|<=|<>|\+=|-=|\*=|/=|%=|&=|^-=|\|\*=' 0:operator
add-highlighter shared/sql/code/ regex \bNULL\b 0:value
add-highlighter shared/sql/code/ regex \b\d+(?:\.\d+)?\b 0:value

]
"#,
        keywords_all = {
            format!(
                "{} {} {} {} {}",
                keywords = KEYWORDS.join(" "),
                operators = OPERATORS.join(" "),
                functions = format!(
                    "{} {} {} {}",
                    MYSQL_FUNCTIONS.join(" "),
                    SQL_SERVER_FUNCTIONS.join(" "),
                    MS_ACCESS_FUNCTIONS.join(" "),
                    ORACLE_FUNCTIONS.join(" ")
                ),
                data_types = format!(
                    "{} {} {}",
                    MYSQL_DATA_TYPES.join(" "),
                    SQL_SERVER_DATA_TYPES.join(" "),
                    MS_ACCESS_DATA_TYPES.join(" ")
                ),
                data_types_fn = format!(
                    "{} {}",
                    MYSQL_DATA_TYPES_FN.join(" "),
                    SQL_SERVER_DATA_TYPES_FN.join(" ")
                ),
            )
        },
        functions = format!(
            "{}|{}|{}|{}",
            MYSQL_FUNCTIONS.join("|"),
            SQL_SERVER_FUNCTIONS.join("|"),
            MS_ACCESS_FUNCTIONS.join("|"),
            ORACLE_FUNCTIONS.join("|")
        ),
        data_types_fn = format!(
            "{}|{}",
            MYSQL_DATA_TYPES_FN.join("|"),
            SQL_SERVER_DATA_TYPES_FN.join("|")
        ),
        keywords = KEYWORDS.join("|"),
        operators = OPERATORS.join("|"),
        data_types = format!(
            "{}|{}|{}",
            MYSQL_DATA_TYPES.join("|"),
            SQL_SERVER_DATA_TYPES.join("|"),
            MS_ACCESS_DATA_TYPES.join("|")
        ),
    )?;

    Ok(buf)
}
