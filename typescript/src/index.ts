import { readFileSync } from "fs";

enum Feedback {
  GREEN = "2",
  YELLOW = "1",
  RED = "0",
}

let words : string[] | null = null;
let memoizedCompareGuess : Record<string, string> = {};

export const FIRST_GUESS = "tares";

export const getWordList = (): string[] => {
  if (!words) {
    const file = readFileSync(__dirname + "/words.txt", "utf-8");
    const wordList = file.split("\n").map((str) => str.trim());
    words = wordList;
  }

  return words;
};


export const filter = (words: string[], guess: string, feedback: string): string[] => {
  const wordArr = words.filter((word) => {
    let yellowPos: { [key: string]: number } = {};

    for (let i = 0; i < word.length; i++) {
      if (
        feedback.charAt(i) === Feedback.RED &&
        guess.charAt(i) === word.charAt(i)
      ) {
        return false;
      }

      if (
        feedback.charAt(i) === Feedback.GREEN &&
        guess.charAt(i) != word.charAt(i)
      ) {
        return false;
      }

      if (
        feedback.charAt(i) === Feedback.YELLOW &&
        word.charAt(i) === guess.charAt(i)
      ) {
        return false;
      }

      if (feedback.charAt(i) === Feedback.YELLOW) {
        const occurances = yellowPos[guess.charAt(i)]
          ? yellowPos[guess.charAt(i)] + 1
          : 1;
        yellowPos[guess.charAt(i)] = occurances;
      }
    }

    const yellowChars = Object.keys(yellowPos);

    for (let j = 0; j < yellowChars.length; j++) {
      const guessChar = yellowChars[j];
      const occurances = yellowPos[guessChar];

      let count = word.split("").filter((x) => x == guessChar).length;

      if (occurances > count) {
        return false;
      }
    }

    return true;
  });

  return wordArr;
};

export const compareGuess = (guess: string, word: string): string => {
  if (memoizedCompareGuess[guess+word]) {
    console.log("true")
    return memoizedCompareGuess[guess+ word];
  }

  const response: (string | null)[] = [null, null, null, null, null];

  const charUsed: Record<string, number> = {};

  for (let i = 0; i < guess.length; i++) {
    if (guess.charAt(i) == word.charAt(i)) {
      response[i] = "2";

      const occurances = charUsed[guess.charAt(i)]
        ? charUsed[guess.charAt(i)] + 1
        : 1;

      charUsed[guess.charAt(i)] = occurances;
    } else if (!word.includes(guess.charAt(i))) {
      response[i] = "0";
    }
  }

  for (let i = 0; i < guess.length; i++) {
    const c = guess[i];

    if (response[i] === null) {
      let occuranceInWord = word.split("").filter((x) => x == c).length;
      if(!charUsed[c] || occuranceInWord < charUsed[c]) {
        response[i] = "1";
        charUsed[c] = charUsed[c] + 1;
      } else {
        response[i] = "0";
      }
    }
  }

  const out = response.join("");
  memoizedCompareGuess[guess+ word] = out

  return out;
};

export const getBestGuess = (wordList: string[]) => {
  let buckets: Record<string, number> = {};

  for (let i = 0; i < wordList.length; i++) {
    const assumedCorrect = wordList[i];
    const tempDiffScores = new Set();

    for (let j = 0; j < wordList.length; j++) {
      const otherWord = wordList[j];

      if (assumedCorrect != otherWord) {
        const tempScore = compareGuess(otherWord, assumedCorrect);
        tempDiffScores.add(tempScore);
      }
    }

    buckets[assumedCorrect] = tempDiffScores.size;
  }

  let best = "";
  let bestScore = -1;

  for (let i = 0; i < wordList.length; i++) {
    const word = wordList[i];
    const tempScore = buckets[word];

    if (tempScore > bestScore) {
      best = word;
      bestScore = tempScore;
    }
  }

  return best;
};
