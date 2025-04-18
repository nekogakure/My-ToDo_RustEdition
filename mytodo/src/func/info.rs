pub fn show_info() {
    println!("\n");
    println!("\t##    ##          #######        #####         ");
    println!("\t##    ##             #           #   ##        ");
    println!("\t###  # # #   #       #     ###   #    #   ###  ");
    println!("\t# #  # #  #  #       #    #  ##  #    ## #  ## ");
    println!("\t# #  # #  # #        #    #   #  #    ## #   # ");
    println!("\t#  ##  #   ##        #    #   #  #    #  #   # ");
    println!("\t#  ##  #   ##        #    #  ##  #   ##  #  ## ");
    println!("\t#      #   #         #     ###   #####    ###  ");
    println!("\t          ##                                   ");
    println!("\t          #                                    ");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("\n\t\tVersion: {}", VERSION);
    println!("\t\tMade by, nekogakure.");
}
