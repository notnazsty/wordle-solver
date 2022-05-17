import { compareGuess, filter, getBestGuess, getWordList } from ".";
import readlineSync from "readline-sync";

console.log(compareGuess("title", "tares"));

console.log(filter(getWordList(), "tares", "20102").length);

// let wordList = getWordList();
// let count = 0;

// wordList = wordList.filter((val) => !val.includes("t")).filter((val) => !val.includes("a")).filter((val) => !val.includes("r")).filter((val) => val.includes("e")).filter((val) => val.includes("s"))

// console.log(wordList.length)

// console.log(getBestGuess(getWordList()));

const words = ["death", "power", "spite", "alone"];

const bucketTest = () => {
  let words = getWordList();
  console.log(getBestGuess(words));
};

// bucketTest();

// for (let i = 0; i < words.length; i++) {
//   let wordList = getWordList();
//   let currentGuess = "tares";
//   let count = 1;
//   let feedback;

//   while (currentGuess != words[i]) {
//     feedback = compareGuess(currentGuess, words[i]);
//     wordList = filter(wordList, currentGuess, feedback);
//     currentGuess = getBestGuess(wordList);
//     count++;
//     console.log(`# of iterations ${count}, with feedback: ${feedback}`);
//   }

//   console.log(currentGuess, count);
// }

const runGame = () => {
  let wordList = getWordList();
  let currentGuess = "tares";
  let count = 1;
  let feedback;

  while (feedback !== "22222") {
    console.log(`\nGuessed ${currentGuess} `);
    feedback = readlineSync
      .question(`Enter feedback (2=GREEN 1=YELLOW 0=GREY):  `)
      .trim();
    wordList = filter(wordList, currentGuess, feedback);
    currentGuess = getBestGuess(wordList);
    console.log(
      `Num possibilities: ${wordList.length}, # of iterations ${count}`
    );
    count++;
  }
};

runGame();
