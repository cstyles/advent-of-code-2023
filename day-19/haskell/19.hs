#!/usr/bin/env runghc

import Data.Char
import Data.List
import Data.Maybe
import Data.Tuple

splitOnce :: Char -> String -> Maybe (String, String)
splitOnce delimiter string =
  let (prefix, rest) = break (== delimiter) string
   in case rest of
        [] -> Nothing
        rest -> Just (prefix, drop 1 rest)

mySplit :: Char -> String -> [String]
mySplit delimiter string =
  case splitOnce delimiter string of
    Nothing -> [string]
    Just (prefix, suffix) -> prefix : mySplit delimiter suffix

data Category = X | M | A | S deriving (Show)

parseCategory :: Char -> Category
parseCategory c =
  case c of
    'x' -> X
    'm' -> M
    'a' -> A
    's' -> S

type Operation = Int -> Int -> Bool

instance Show Operation where
  show (<) = show '<'
  show (>) = show '>'

parseOperation :: Char -> Operation
parseOperation '<' = (<)
parseOperation '>' = (>)

data Rule = Rule
  { category :: Category,
    operation :: Operation,
    value :: Int,
    outcome :: Outcome
  }
  deriving (Show)

parseValue :: [Char] -> Int
parseValue = foldl' (\total num -> total * 10 + digitToInt num) 0

parseRule :: String -> Rule
parseRule string =
  case string of
    [] -> error "bad input"
    category' : operation' : rest ->
      let operation = parseOperation operation'
       in let category = parseCategory category'
           in let (value', outcome') = fromJust $ splitOnce ':' rest
               in let value = parseValue value'
                   in let outcome = parseOutcome outcome'
                       in Rule {category, operation, value, outcome}

data Outcome = Accept | Reject | Redirect String deriving (Show)

parseOutcome :: String -> Outcome
parseOutcome "A" = Accept
parseOutcome "R" = Reject
parseOutcome name = Redirect name

data Workflow = Workflow
  { name :: String,
    rules :: [Rule],
    default_ :: Outcome
  }
  deriving (Show)

parseWorkflow :: String -> Workflow
parseWorkflow line =
  let (name, rulesWithSuffix) = fromJust $ splitOnce '{' line
   in let rulesWithoutSuffix = takeWhile (/= '}') rulesWithSuffix
       in let (rulesWithoutOutcome, outcome') = fromJust $ unsnoc $ mySplit ',' rulesWithoutSuffix
           in let outcome = parseOutcome outcome'
               in let rules = map parseRule rulesWithoutOutcome
                   in Workflow {name, rules, default_ = outcome}

workflowAccepts :: Workflow -> [(String, Workflow)] -> Part -> Bool
workflowAccepts workflow workflows part =
  case (firstMap (\rule -> ruleOutcome rule part)) (rules workflow) of
    Just outcome ->
      case outcome of
        Accept -> True
        Reject -> False
        Redirect name ->
          let new_workflow = fromJust $ lookup name workflows
           in workflowAccepts new_workflow workflows part
    Nothing ->
      case (default_ workflow) of
        Accept -> True
        Reject -> False
        Redirect name ->
          let new_workflow = fromJust $ lookup name workflows
           in workflowAccepts new_workflow workflows part

data Part = Part {x :: Int, m :: Int, a :: Int, s :: Int} deriving (Show)

parsePart :: String -> Part
parsePart string =
  let (_, inner') = fromJust $ uncons string
   in let (inner, _) = fromJust $ unsnoc inner'
       in let (x', rest'') = fromJust $ splitOnce ',' inner
           in let (m', rest') = fromJust $ splitOnce ',' rest''
               in let (a', s') = fromJust $ splitOnce ',' rest'
                   in Part {x = parseValue $ drop 2 x', m = parseValue $ drop 2 m', a = parseValue $ drop 2 a', s = parseValue $ drop 2 s'}

partTotal :: Part -> Int
partTotal part = sum [x part, m part, a part, s part]

getCategory :: Category -> Part -> Int
getCategory category part =
  case category of
    X -> x part
    M -> m part
    A -> a part
    S -> s part

ruleOutcome :: Rule -> Part -> Maybe Outcome
ruleOutcome rule part =
  let partValue = getCategory (category rule) part
   in if (operation rule) partValue (value rule)
        then Just (outcome rule)
        else Nothing

sections :: String -> ([String], [String])
sections input =
  partition (not . isPrefixOf "{") $ lines input

-- Finds the first item in a list that returns Just when f is applied to it.
-- Returns the Just value if such an item exists. Otherwise returns Nothing.
firstMap :: (a -> Maybe b) -> [a] -> Maybe b
firstMap f list =
  case list of
    [] -> Nothing
    head : tail ->
      case f head of
        Nothing -> firstMap f tail
        otherwise -> otherwise

main = do
  input <- getContents
  let (workflows', parts') = sections input
   in let workflows = map (\w -> (name w, w)) $ map parseWorkflow $ filter (not . null) workflows'
       in let parts = map parsePart parts'
           in do
                print $ part1 workflows parts

part1 :: [(String, Workflow)] -> [Part] -> Int
part1 workflows parts =
  let inWorkflow = fromJust $ lookup "in" workflows
   in foldl' (\acc part -> if workflowAccepts inWorkflow workflows part then acc + partTotal part else acc) 0 parts
