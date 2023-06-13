use crate::utils::{into_v32, remove_whitespaces};
use crate::render_to_html_with_default_options;

fn blockquote_samples() -> Vec<(String, String)> {
    let result = vec![
        ("
>>> 3
", "
<blockquote><blockquote><blockquote>3</blockquote></blockquote></blockquote>
"), ("
> 1
>> 2
>>> 3
>>> > 4
>>>>> 5
> > > > > > 6
> > > > > > > 7
> > > > > > > > 8
> > > > > > > > > 9
> > > > > > > > > > 10
> > > > > > > > > > > 11
> > > > > > > > > > > > 12
", "
<blockquote>1
    <blockquote>2
        <blockquote>3
            <blockquote>4
                <blockquote>5
                    <blockquote>6
                        <blockquote>7
                            <blockquote>8
                                <blockquote>9
                                    <blockquote>10
                                        <blockquote>11
                                            <blockquote>12</blockquote>
                                        </blockquote>
                                    </blockquote>
                                </blockquote>
                            </blockquote>
                        </blockquote>
                    </blockquote>
                </blockquote>
            </blockquote>
        </blockquote>
    </blockquote>
</blockquote>
"), ("
> >>  >>>   >>>> 10
", "
<blockquote><blockquote><blockquote><blockquote><blockquote><blockquote><blockquote><blockquote><blockquote><blockquote>10</blockquote></blockquote></blockquote></blockquote></blockquote></blockquote></blockquote></blockquote></blockquote></blockquote>
"), ("
> >>  >>>   >>>>    >>>>> 10
", "
<blockquote><blockquote><blockquote><blockquote><blockquote><blockquote><blockquote><blockquote><blockquote><blockquote> &gt;&gt;&gt;&gt;&gt; 10</blockquote></blockquote></blockquote></blockquote></blockquote></blockquote></blockquote></blockquote></blockquote></blockquote>
"), ("
>     > 1
", "
<blockquote>  &gt; 1</blockquote>
"), ("
   >> 2
", "
<blockquote><blockquote>2</blockquote></blockquote>
"), ("
> 1
>
> 1
", "
<blockquote>1  1 </blockquote>
"), ("
> 1
> 
> 1
", "
<blockquote>1  1 </blockquote>
"), ("
>>> 3
>>>
>>> 3
", "
<blockquote><blockquote><blockquote>3  3 </blockquote></blockquote></blockquote>
"), ("
>>> 3
>>> 
>>> 3
", "
<blockquote><blockquote><blockquote>3  3 </blockquote></blockquote></blockquote>
"), ("
>1
", "
<blockquote>1 </blockquote>
"), ("
>>> 3
> 3
>> 3
>>>> 4
>> 4
", "
<blockquote><blockquote><blockquote>3 3 3 <blockquote>4 4 </blockquote></blockquote></blockquote></blockquote>
"), ("
>>> 3

> 1

>> 2

>>>> 4

>> 2
", "
<blockquote><blockquote><blockquote>3 </blockquote></blockquote></blockquote>

<blockquote>1 </blockquote>

<blockquote><blockquote>2 </blockquote></blockquote>

<blockquote><blockquote><blockquote><blockquote>4 </blockquote></blockquote></blockquote></blockquote>

<blockquote><blockquote>2 </blockquote></blockquote>
"), ("
paragraph
> blockquote
blockquote

paragraph

> blockquote

paragraph
", "
<p>paragraph</p>

<blockquote>blockquote blockquote </blockquote>

<p>paragraph</p>

<blockquote>blockquote </blockquote>

<p>paragraph</p>
"), ("
> 1
- 1
> 1
- 1
> 1
- 1
", "
<blockquote>1 </blockquote>

<ul><li>1</li></ul>

<blockquote>1 </blockquote>

<ul><li>1</li></ul>

<blockquote>1 </blockquote>

<ul><li>1</li></ul>
"), ("
> 1
>>> 3
>>>>> 5
>>>>>>> 7
", "
<blockquote>1 
    <blockquote>
        <blockquote>3 
            <blockquote>
                <blockquote>5 
                    <blockquote>
                        <blockquote>7 </blockquote>
                    </blockquote>
                </blockquote>
            </blockquote>
        </blockquote>
    </blockquote>
</blockquote>
"), ("
>>>>>>> 7
>>>>> 7
>>> 7
> 7
", "
<blockquote><blockquote><blockquote><blockquote><blockquote><blockquote><blockquote>
    7 7 7 7
</blockquote></blockquote></blockquote></blockquote></blockquote></blockquote></blockquote>
"), ("
interrupt
paragraph
> 1
> 1
", "
<p>interrupt paragraph</p>

<blockquote>1 1 </blockquote>
"), ("
> *inline* ~_elements_~
", "
<blockquote><em>inline</em> <u>elements</u> </blockquote>
")
    ];

    result.into_iter().map(
        |(case, answer)| (case.to_string(), answer.to_string())
    ).collect()
}

#[test]
fn blockquote_test() {
    for (md, html) in blockquote_samples().iter() {
        let rendered = render_to_html_with_default_options(md);

        if remove_whitespaces(&into_v32(&rendered)) != remove_whitespaces(&into_v32(html)) {
            panic!("{md} \n\n {rendered}");
        }

    }

}