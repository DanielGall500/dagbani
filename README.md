# Dagbani Phonological Toolkit

This repository contains tools and scripts developed in the Rust programming language for the computational analysis of Dagbani phonology, specifically focused on vowel Advanced Tongue Root (ATR) properties and moraic structure. These tools were designed to help researchers verify phonological theories in under-studied languages by analyzing larger datasets.

#### What is "Advanced Tongue Root"?
Advanced Tongue Root (ATR) is a feature of some languages that affects how vowels are pronounced. It refers to whether the root of the tongue moves forward (advanced) or stays back in the throat when making a vowel sound. 
- **[+ATR] vowels**: The tongue moves slightly forward, making the vowels sound "tenser" or more open (like "e" or "i").
- **[–ATR] vowels**: The tongue stays back, making the vowels sound "laxer" or more closed (like "a" or "ɛ").

This distinction helps create different vowel sounds and affects how words are pronounced in certain languages.

## Research Overview

In recent years, computational methods have become essential for the analysis of linguistic data. This project provides open-source tools for researchers to explore Dagbani phonological patterns, particularly for investigating the occurrence of ATR vowels in varying moraic structures. The primary tools developed for this project include:

1. **Dagbani pre-processing scripts** – To handle Dagbani text data, including IPA symbols.
2. **Phonological word to CV-structure converter** – A script that parses phonological words into their consonant-vowel (CV) structure.
3. **CV-structure to moraic-structure converter** – A Finite State Transducer (FST) that maps CV structures to their corresponding moraic structures.

#### 1. Dagbani Pre-processing Scripts
These scripts are designed to clean, normalise, and prepare Dagbani text data for further computational processing, particularly focusing on IPA (International Phonetic Alphabet) symbols. The pre-processing handles tasks such as:

Normalization: Ensuring that characters, especially non-ASCII ones (like IPA symbols), are standardized (e.g., converting various forms of the same character to a unified format).
Transliteration: Converting certain phonetic representations in Dagbani into IPA symbols or other required formats.
Noise removal: Filtering out non-textual elements or irrelevant characters (e.g diacritics) that could interfere with the analysis.

#### 2. Phonological Word to CV-Structure Converter
This script takes Dagbani phonological words as input and outputs their Consonant-Vowel (CV) structure. Every syllable can be broken down into consonants (C) and vowels (V), and this structure provides a simplified representation of how words are constructed.

For example, the word "ba" (meaning father in Dagbani) would be parsed as CV, where "b" is a consonant and "a" is a vowel. The converter handles:
* Parsing phonological input: Breaking down words into their phonetic components (e.g., IPA symbols).
* Pattern matching: Identifying which parts of the word are consonants and which are vowels.
* CV output generation: Producing a sequence like CVC, CVV, etc., based on the input word’s structure.

#### 3. CV-Structure to Moraic-Structure Converter
This component is a Finite State Transducer (FST) that maps the CV structure of words to their corresponding moraic structure. In prosodic phonology, a mora is a unit that measures syllable weight, which is important for understanding stress, rhythm, and timing in speech. Here's how it works:

Input CV structures: The converter takes the CV representation (from the previous step).
Mapping rules: It applies specific linguistic rules to assign a moraic weight to each segment. For example, certain vowels or long vowels may carry more weight (i.e., more morae).
Output moraic structure: The FST outputs the moraic structure, often represented as sequences of μ (moras). For instance, a CV syllable might be represented as one mora (μ), while a CVC syllable might be two morae (μμ).

![Dagbani Finite State Transducer](img/DagbaniFST.png)

An example of the FST converting the Dagbani word *mɛ́rígí* ("destroy with pressure") to its moraic structure:

| Step  | 1  | 2    | 3   | 4    | 5   | 6    | 7   | 8   |
|-------|----|------|-----|------|-----|------|-----|-----|
| Input | START  | C\_ons | V | C\_ons | V | C\_ons | V | END |
| States | START | C\_ons | V | C\_ons | V | C\_ons | V | END |
| Output | λ | λ | μ | λ | μ | λ | μ | λ |

*Note: The placeholder lambdas (λ) represent no mora being added and are removed from the final output, yielding a moraic makeup of three μ's.*

## Results

The computational analysis produced tables summarising the vowel occurrences categorized by ATR values across various CV and moraic structures. The key findings confirmed previous phonological analyses of Dagbani and provided further insights into the role of foot structure in determining ATR vowel occurrence. For instance, the [+ATR] vowel [u] only appeared in unfooted positions, while [--ATR] vowels such as [Ʊ] were restricted to footed positions.

Key results are summarized in the following tables:
| **CV Structure** | **Present [--ATR]** | **Present [+ATR]** | **Absent [--ATR]** | **Absent [+ATR]** |
|------------------|---------------------|--------------------|--------------------|-------------------|
| CV               | a                   | e, i, u, o          | ɨ, ɛ, ɔ, ʊ         | ə                 |
| CVC              | ʊ, a, ɨ             |                    | ɛ, ɔ               | ɨ, e, o, u, ə      |
| CCV              | ɛ                   | i                  | ɔ, ʊ               | ɨ, e, o, a, u, ə   |
| CVCV             | ɔ, ʊ, ɨ             | i, e, o            | ɛ                  | a, u, ə           |
| CVVC             | ɛ                   | i                  | ɔ, ʊ, ɨ            | e, a, o, u, ə      |
| CVVV             |                     | i, o               | ɛ, ɔ, ʊ, ɨ         | e, a, u, ə         |
| CVCVC            | ʊ                   | i                  | ɛ, ɔ, ɨ            | e, o, a, u, ə      |
| CVVCV            |                     | a, i, e, o         | ɛ, ɔ, ʊ, ɨ         | u, ə              |
| CVCVCV           | ɔ, ʊ, ɛ, ɨ          | i, o               | ɛ, ɔ, ʊ, ɨ         | e, a, u, ə         |

*Vowel ATR value occurrences in dataset after computational analysis of CV structure.*

| **Mora Count**   | **[--ATR]**                       | **[+ATR]**          |
|------------------|-----------------------------------|---------------------|
| $\mu$            | a, ɛ                              | e, i, o, u          |
| $\mu\mu$         | ɔ, a, ɨ, ʊ                        | e, i, o             |
| $\mu\mu\mu$      | ɔ, a, ɨ, ʊ (footed), ɛ            | e, i, o             |

*Computational methodology applied to data using the CV → μ FST.*

## How to Use

1. Clone the repository:
   ```
   git clone https://github.com/DanielGall500/dagbani-phonological-toolkit
   ```
2. Install Rust from [rust-lang.org](https://www.rust-lang.org/).
3. Run the tools on your Dagbani dataset by following the instructions in the `src` folder.

## Future Work

- Expand the dataset to include more verb roots and other word categories.
- Improve the FST to account for more complex phonological phenomena.
- Collaborate with researchers in phonology and computational linguistics to refine the tools and analyses.

## References

- Hudu, F. (2006). "Advanced Tongue Root Vowel Harmony in Dagbani." PhD Dissertation.
- Ermolaeva, E. (2019). "Quantitative Methods in Phonological Analysis."

---

Feel free to contribute or report issues via [GitHub](https://github.com/DanielGall500/dagbani-phonological-toolkit).
