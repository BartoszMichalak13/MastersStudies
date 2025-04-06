use std::env;

fn find_genes(dna: &str) -> Vec<&str> {
  let stop_codons = ["TAA", "TAG", "TGA"];
  let mut result = vec![];
  let chars: Vec<char> = dna.chars().collect();
  let mut i = 0;

  while i + 2 < chars.len() {
    if &chars[i..i + 3].iter().collect::<String>() == "ATG" {
      let start = i;
      i += 3;

      while i + 2 < chars.len() {
        let codon = &chars[i..i + 3].iter().collect::<String>();
        if stop_codons.contains(&codon.as_str()) {
          let gene = &dna[start..i + 3];
          let u = &dna[start + 3..i];

          if u.len() >= 30
            && !["ATG", "TAA", "TAG", "TGA"].iter().any(|&x| u.contains(x))
          {
            result.push(gene);
          }
          break;
        }
        // i += 3;
        i += 1;
      }
    } else {
      i += 1;
    }
  }

  result
}

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();

  let dna = if args.is_empty() {
    // "ATGCGATAGTAAATGCTAGTAGGATGCGTAA"  // Przykładowa sekwencja DNA
    // "ATGCGATAGTAAATGCTAGTAGGATGCGTAA"  // Przykładowa sekwencja DNA
    // "ATGCGATAGTAAATGCTAGTAGGATGCGTAA"  // Przykładowa sekwencja DNA
    // "ATGCGATAGTAAATGCTAGTAGGATGCGTAA"  // Przykładowa sekwencja DNA
    "ATGTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTAA"  // Przykładowa sekwencja DNA
  } else {
    &args[0]
  };

  let genes = find_genes(dna);

  println!("Znalezione geny:");
  for gene in genes {
    println!("{}", gene);
  }
}