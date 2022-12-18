#![deny(missing_docs)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// FUNCTIONS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Integrates a function from `a` to `b`.
/// Uses the Tanh-Sinh quadrature over [-1, +1]
/// and then transforms to an integral over [a, b].
pub fn integrate<F>(f: F, a: f64, b: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let c = 0.5 * (b - a);
    let d = 0.5 * (a + b);
    c * tanhsinh(|x| {
        let out = f(c * x + d);
        if out.is_finite() {
            out
        } else {
            0.0
        }
    })
}

fn tanhsinh<F>(f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let mut integral = 0.0;

    for i in 0..100 {
        integral += WEIGHTS[i] * f(ABSCISSAE[i])
    }

    integral
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ABSCISSAE & WEIGHTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Nodes and weights from: https://keisan.casio.com/exec/system/1285151216

/// Abscissae: the nodes for the sum evaluation.
pub const ABSCISSAE: [f64; 100] = [
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -1.0,
    -0.99999999999999999999999999999999999999999999899512,
    -0.99999999999999999999999999999999999999978245507855,
    -0.99999999999999999999999999999999998890287307099258,
    -0.99999999999999999999999999999984185753238747824951,
    -0.99999999999999999999999999926833162843776103387285,
    -0.99999999999999999999999874517118251905899572987687,
    -0.99999999999999999999910326246517911676180488146856,
    -0.9999999999999999997039324425832433076593182271234,
    -0.9999999999999999505317162468729600280358231878204,
    -0.99999999999999546692519196928846990821504893567577,
    -0.99999999999975542588696783126910635562547483071579,
    -0.99999999999172844011012792856817661827168148145848,
    -0.9999999998146702729797820220064891699834983138399,
    -0.9999999971113436836255458019712433402444030704157,
    -0.99999996729784596679312379310007422805776128889163,
    -0.99999972065454365550920289324738326328835621352624,
    -0.99999813780532109535465470589317286826733797862984,
    -0.99999001914385679451044606398085404161020435782687,
    -0.99995584097869348062061656286012045808240002863364,
    -0.99983491316226506087268272063705286063189355729644,
    -0.99946763580399903702887559121611627293042256385811,
    -0.99849193873160297908589098235898161135160868504915,
    -0.99618677158524374076004415125282528023843559326676,
    -0.99127256023655099619654783112053733636604534357967,
    -0.98170156597387694717623566556856086993008164206292,
    -0.96449537993111148758148463374572739153334671321554,
    -0.93570868874013988919597118002367718162849346837222,
    -0.89061315397063692385699214001450546511207730479522,
    -0.82419097246841227144126417840605113640370738508653,
    -0.73198295497756851419790032758214254640240315379651,
    -0.61122893304755064073254005842603095995762649476587,
    -0.46207234244563718559529318889800812077551400513962,
    -0.28843516389039866269534159325094516554486834270644,
    -0.098120697049078749124870172108653391424710439434287,
    0.098120697049078749124870172108653391424710439434255,
    0.28843516389039866269534159325094516554486834270641,
    0.46207234244563718559529318889800812077551400513959,
    0.61122893304755064073254005842603095995762649476585,
    0.73198295497756851419790032758214254640240315379649,
    0.82419097246841227144126417840605113640370738508652,
    0.89061315397063692385699214001450546511207730479521,
    0.93570868874013988919597118002367718162849346837222,
    0.96449537993111148758148463374572739153334671321553,
    0.98170156597387694717623566556856086993008164206292,
    0.99127256023655099619654783112053733636604534357967,
    0.99618677158524374076004415125282528023843559326676,
    0.99849193873160297908589098235898161135160868504915,
    0.9994676358039990370288755912161162729304225638581,
    0.99983491316226506087268272063705286063189355729644,
    0.99995584097869348062061656286012045808240002863364,
    0.99999001914385679451044606398085404161020435782687,
    0.99999813780532109535465470589317286826733797862984,
    0.99999972065454365550920289324738326328835621352624,
    0.99999996729784596679312379310007422805776128889163,
    0.9999999971113436836255458019712433402444030704157,
    0.9999999998146702729797820220064891699834983138399,
    0.99999999999172844011012792856817661827168148145848,
    0.99999999999975542588696783126910635562547483071579,
    0.99999999999999546692519196928846990821504893567577,
    0.9999999999999999505317162468729600280358231878204,
    0.9999999999999999997039324425832433076593182271234,
    0.99999999999999999999910326246517911676180488146856,
    0.99999999999999999999999874517118251905899572987687,
    0.99999999999999999999999999926833162843776103387285,
    0.99999999999999999999999999999984185753238747824951,
    0.99999999999999999999999999999999998890287307099258,
    0.99999999999999999999999999999999999999978245507855,
    0.99999999999999999999999999999999999999999999899512,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
];

/// Weights for the sum evaluation.
pub const WEIGHTS: [f64; 100] = [
    1.3847723087524135159260744218630568063772513592071e-334,
    4.5756179301783380542064862727246219754632103659228e-295,
    3.3169944625703463637387782530270690009809879240363e-260,
    1.8652126224951733884453636550942024721225683840374e-229,
    2.4791216720520923925696675363929060728214952955776e-202,
    2.0815374482956259750753017499196764459630281983907e-178,
    2.6282020113568824737123769403073465347791523381172e-157,
    1.0726298856549832272396144493597055697832359316665e-138,
    2.7794823052616371780914961913666624626831818417554e-122,
    8.2964188446635157384004377606271824113537974154763e-108,
    4.824667051120648130845484619944936650655941141437e-95,
    8.690880649537158675864343969472497080962687057723e-84,
    7.3003846236041368435598450086562205919512296572206e-74,
    4.1026627498062048670416457609295032603079825389572e-65,
    2.1209277150278448662383387954406343356590594238573e-57,
    1.3358249934060332062159956908242834686474504169951e-50,
    1.3134068783205992296027430114580089253423743257675e-44,
    2.5088078163950786407797045866896052382330272929549e-39,
    1.1291938670594328251356304303248449314368425467049e-34,
    1.4198949758810437733168935226419360421120440122761e-30,
    5.7967544218961547353045039137809232143639986937589e-27,
    8.7727329139273969776551704750443062384649506895934e-24,
    5.5324471251352894000218918131022732795290060422407e-21,
    1.6120262304040780830730052389117544142428307456616e-18,
    2.3772412457440434762695491856430763011169616937644e-16,
    1.9228718774555729343911853992459031296538836720311e-14,
    9.1587863788110880159548116891240547848925270042663e-13,
    2.7350196856077822020082758495592354346908249863867e-11,
    5.412036348700473900675895348302925795817071091131e-10,
    7.4520954951991127953680087546349491417334243634587e-9,
    7.4556433532810560601790151910739958630845907757591e-8,
    5.6309352582104192985963126106848644303636606857643e-7,
    3.3208922218874238326217853388687449346881388543946e-6,
    1.5758692556974203167002602296375658198054279958889e-5,
    6.1789408791972789617314197389042406902327112241017e-5,
    2.0496122229359242117473065854406070967065119708872e-4,
    5.8730888502323620478007763455303343927599280250793e-4,
    0.0014808565224807768260219225526629908566057899472652,
    0.0033391089061553599313146952183655387509394530977121,
    0.0068277044161554556976442551186034713196296825790514,
    0.012809851589261179031558854539477331748397174975569,
    0.022262908367071660733552809539524134213785943000127,
    0.036106623280003482363473668562104202273792877834151,
    0.054935001719810428299022548756800417885928283003707,
    0.078672104392875153646481324221588993392043777075097,
    0.10622501864477554727451852720822012168687600659496,
    0.13527485447886908523917288682926424798111246275939,
    0.16238710772598652822378626776893197126105401310686,
    0.18357084195528500238596745121511094521604055152081,
    0.19523423322883864982883687771732241534841980322711,
    0.19523423322883864982883687771732241534841980322711,
    0.18357084195528500238596745121511094521604055152081,
    0.16238710772598652822378626776893197126105401310687,
    0.13527485447886908523917288682926424798111246275939,
    0.10622501864477554727451852720822012168687600659496,
    0.0786721043928751536464813242215889933920437770751,
    0.05493500171981042829902254875680041788592828300371,
    0.036106623280003482363473668562104202273792877834153,
    0.022262908367071660733552809539524134213785943000129,
    0.01280985158926117903155885453947733174839717497557,
    0.0068277044161554556976442551186034713196296825790521,
    0.0033391089061553599313146952183655387509394530977126,
    0.0014808565224807768260219225526629908566057899472654,
    5.8730888502323620478007763455303343927599280250801e-4,
    2.0496122229359242117473065854406070967065119708876e-4,
    6.1789408791972789617314197389042406902327112241028e-5,
    1.5758692556974203167002602296375658198054279958893e-5,
    3.3208922218874238326217853388687449346881388543957e-6,
    5.6309352582104192985963126106848644303636606857658e-7,
    7.455643353281056060179015191073995863084590775762e-8,
    7.4520954951991127953680087546349491417334243634618e-9,
    5.4120363487004739006758953483029257958170710911333e-10,
    2.7350196856077822020082758495592354346908249863882e-11,
    9.1587863788110880159548116891240547848925270042718e-13,
    1.9228718774555729343911853992459031296538836720322e-14,
    2.3772412457440434762695491856430763011169616937658e-16,
    1.6120262304040780830730052389117544142428307456631e-18,
    5.5324471251352894000218918131022732795290060422456e-21,
    8.772732913927396977655170475044306238464950689607e-24,
    5.7967544218961547353045039137809232143639986937681e-27,
    1.4198949758810437733168935226419360421120440122783e-30,
    1.1291938670594328251356304303248449314368425467096e-34,
    2.5088078163950786407797045866896052382330272929471e-39,
    1.3134068783205992296027430114580089253423743257675e-44,
    1.3358249934060332062159956908242834686474504169978e-50,
    2.1209277150278448662383387954406343356590594238716e-57,
    4.1026627498062048670416457609295032603079825389378e-65,
    7.3003846236041368435598450086562205919512296572206e-74,
    8.6908806495371586758643439694724970809626870577547e-84,
    4.8246670511206481308454846199449366506559411414896e-95,
    8.2964188446635157384004377606271824113537974154161e-108,
    2.7794823052616371780914961913666624626831818417554e-122,
    1.0726298856549832272396144493597055697832359316766e-138,
    2.6282020113568824737123769403073465347791523381667e-157,
    2.0815374482956259750753017499196764459630281983711e-178,
    2.4791216720520923925696675363929060728214952955776e-202,
    1.8652126224951733884453636550942024721225683840667e-229,
    3.316994462570346363738778253027069000980987924123e-260,
    4.5756179301783380542064862727246219754632103658749e-295,
    1.3847723087524135159260744218630568063772513592071e-334,
];

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use crate::assert_approx_equal;

    use super::*;

    #[test]
    fn test_quadrature() {
        fn f(x: f64) -> f64 {
            (x.sin()).exp()
        }

        let integral = integrate(f, 0.0, 5.0);

        assert_approx_equal!(integral, 7.189119253631, 1e-8);
    }
}