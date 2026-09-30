use nuclear_assisted_smr::final_design::{HEAT_MWTH,REACTOR_MWTH,NISHIHARA_IHX_MWTH};
fn main(){
 let residual=REACTOR_MWTH-HEAT_MWTH;
 println!(r#"\\begin{{figure}}[htbp]\\centering"#);
 println!(r#"\\begin{{tikzpicture}}[x=0.0105cm,y=1cm]"#);
 println!(r#"\\draw[->] (0,0)--(640,0) node[right]{{MWth}};"#);
 println!(r#"\\draw[very thick] (0,1.35)--({:.1},1.35) node[midway,above]{{reactor rating {:.0} MWth}};"#,REACTOR_MWTH,REACTOR_MWTH);
 println!(r#"\\draw[very thick] (0,0.65)--({:.1},0.65) node[midway,above]{{process duty {:.1}}};"#,HEAT_MWTH,HEAT_MWTH);
 println!(r#"\\draw[dashed] ({:.1},0.25)--({:.1},1.75) node[above]{{source heat branch {:.0}}};"#,NISHIHARA_IHX_MWTH,NISHIHARA_IHX_MWTH,NISHIHARA_IHX_MWTH);
 println!(r#"\\draw[<->] ({:.1},0.2)--({:.1},0.2) node[midway,below]{{remaining {:.1} MWth thermal; no power claim}};"#,HEAT_MWTH,REACTOR_MWTH,residual);
 println!(r#"\\end{{tikzpicture}}"#);
 println!(r#"\\caption{{Rust-generated thermal-capacity comparison from canonical final-design constants. Remaining reactor thermal capacity is not converted into an electricity claim.}}"#);
 println!(r#"\\label{{fig:rust-heat-flow}}\\end{{figure}}"#);
}