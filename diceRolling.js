

function roll(req){
    if(typeof req !== "string"){
        return "Mollux! You don goofed!"
    }

    // Splits request into segments. Example "3d6+3+5d2" => ["3d6", "+3", "+5d2"]
    var segments = req.replace(/\s/g, '').match(/([+\-*/]?\d{0,}d\d{1,})|([+\-*/]\d{1,})/gi)

    var i = -1
    var results = []
    var curResult = 0
    var curMod = 1
    segments.forEach(e => {
        if (e.match(/([+\-*/]?\d{0,}d\d{1,})/gi)){ // xdy
            if (e.substring(0, 1)) curMod = -1
            // TODO: Do stuff
        } else if (e.match(/([+\-*/]\d{1,})/gi)){ // Modifier
            // TODO: Do stuff
        }
    })
}