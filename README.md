# Fitch
Fitch is a small command-line editor for writing natural deduction
(propositional logic) proofs. However, the program hasn't been extensively tested and was just written to get a chance to play around with propositional logic and the wonderful `chumsky` parser combinator library. But it can be rather helpful for generating LaTeX and avoid having to manually typeset proofs.

See the video for a short demo:

[demo.webm](https://github.com/adelhult/fitch/assets/11508459/d02ca625-7e8c-43ac-8e85-24fc15a097d9)

## Usage
Assuming you have [Rust installed](https://www.rust-lang.org/learn/get-started). Clone the repo and run:
```sh
cd fitch_cli
cargo run
```
You can then start using fitch by typing a *command* followed by a series of arguments:
* `premise <formula>` - Add a new premise
* `copy <step index>` - Copy a previously proven formula
* `assume <formula>` - Create a new proof box (sub-proof) with some assumption
* `discharge` - Close a proof box 
* `rule <rule name> <rule arguments...>` - Apply a rule given some step indices. See a list of rules below.
* `undo` - Undo the latest step
* `quit` - Quit the program
* `help` - Print a help message
* `latex` - Generate LaTeX code to typeset your proof

## Rules
The rules are the same as those presented in Huth and Ryan *Logic in Computer Science*. (With the expection that $\neg \varphi$ is encoded as $\varphi \to \bot$).

<table>
  <tr>
    <th>Name</th>
    <th>Arguments</th>
    <th>Explanation</th>
  </tr>
  <tr>
    <td>and_i</td>
    <td>StepIndex StepIndex</td>
    <td>
      <pre>
        phi     psi
        -----------
        phi  ^  psi
      </pre>
    </td>
  </tr>
  <tr>
    <td>and_e_lhs</td>
    <td>StepIndex</td>
    <td>
      <pre>
        phi  ^  psi
        -----------
            phi
      </pre>
    </td>
  </tr>
  <tr>
    <td>and_e_rhs</td>
    <td>StepIndex</td>
    <td>
      <pre>
        phi  ^  psi
        -----------
            psi
      </pre>
    </td>
  </tr>
  <tr>
    <td>or_i_lhs</td>
    <td>StepIndex Prop</td>
    <td>
      <pre>
            phi
        ------------
        phi \/ psi
      </pre>
    </td>
  </tr>
  <tr>
    <td>or_i_rhs</td>
    <td>Prop StepIndex</td>
    <td>
      <pre>
            psi
        ------------
        phi \/ psi
      </pre>
    </td>
  </tr>
  <tr>
    <td>or_e</td>
    <td>StepIndex StepIndex StepIndex</td>
    <td>
      <pre>
        phi \/ psi   [phi]...chi   [psi]...chi
        -------------------------------------
                        chi
      </pre>
    </td>
  </tr>
  <tr>
    <td>neg_i</td>
    <td>StepIndex</td>
    <td>
      <pre>
        [phi]...bottom
        ----------------
            neg phi
      </pre>
    </td>
  </tr>
  <tr>
    <td>neg_e</td>
    <td>StepIndex StepIndex</td>
    <td>
      <pre>
        phi    neg phi
        ----------------
            bottom
      </pre>
    </td>
  </tr>
  <tr>
    <td>imply_i</td>
    <td>StepIndex</td>
    <td>
      <pre>
        [phi]...psi
        -------------
            psi
      </pre>
    </td>
  </tr>
  <tr>
    <td>imply_e</td>
    <td>StepIndex StepIndex</td>
    <td>
      <pre>
        phi      phi -> psi
        ---------------------
                psi
      </pre>
    </td>
  </tr>
  <tr>
    <td>bottom-e</td>
    <td>StepIndex Prop</td>
    <td>
      <pre>
          bottom
        ------------
            phi
      </pre>
    </td>
  </tr>
  <tr>
    <td>neg_neg_e</td>
    <td>StepIndex</td>
    <td>
      <pre>
        neg (neg phi)
        ---------------
            phi
      </pre>
    </td>
  </tr>
  <tr>
    <td>mt</td>
    <td>StepIndex StepIndex</td>
    <td>
      <pre>
        phi -> psi    neg psi
        ------------------------
                neg phi
      </pre>
    </td>
  </tr>
  <tr>
    <td>neg_neg_i</td>
    <td>StepIndex</td>
    <td>
      <pre>
            phi
        ---------------
        neg (neg phi)
      </pre>
    </td>
  </tr>
  <tr>
    <td>PBC</td>
    <td>StepIndex</td>
    <td>
      <pre>
        [not phi]...bottom
        --------------------
                phi
      </pre>
    </td>
  </tr>
  <tr>
    <td>LEM</td>
    <td>Prop</td>
    <td>
      <pre>
        ----------------
        phi \/ neg phi
      </pre>
    </td>
  </tr>
</table>
