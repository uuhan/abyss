local lume = require("lume")
local CFFEX = { "IC", "IF", "IH", "T", "TF", "TS", "IM", "TL" }

local GFEX = { "si", "lc" }

local SHFE = {
  "ao", "br", "au", "ag", "cu", "ni",
  "ru", "fu", "bu", "rb", "ss", "sp",
  "hc", "wr", "al", "pb", "zn", "sn",
}

local INE = { "sc", "lu", "nr", "bc", "ec" }

local DCE = {
  "a", "b", "bb", "c", "cs", "eb",
  "eg", "fb", "i", "j", "jd", "jm",
  "l", "lh", "m", "p", "pg", "pp",
  "rr", "v", "y",
}

local CZCE = {
  "AP", "CF", "CJ", "CY", "FG",
  "JR", "LR", "MA", "OI", "PF",
  "PK", "PM", "RI", "RM", "RS",
  "SA", "SF", "SM", "SR", "TA",
  "UR", "WH", "ZC", "SH",
}

return lume.concat(CFFEX, GFEX, SHFE, INE, DCE, CZCE)
