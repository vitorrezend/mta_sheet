use std::fmt;
use std::rc::Rc;

/// Callback robusto e seguro baseado em `Rc<dyn Fn(In) -> Out>`.
///
/// Diferente do `leptos::Callback` (que depende internamente de `StoredValue` na runtime reativa
/// do Leptos e panica com `'could not get stored value'` ao ser invocado apos o descarte ou
/// reavaliacao do Scope pai), o `SafeCallback` mantem a closure na Heap com contagem de
/// referencias `Rc`, operando de forma 100% segura e livre de panics em qualquer contexto
/// (SSR, hidratacao CSR, tarefas assincronas `spawn_local`, temporizadores e eventos DOM).
#[derive(Clone)]
pub struct SafeCallback<In, Out = ()>(Rc<dyn Fn(In) -> Out>);

impl<In, Out> SafeCallback<In, Out> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(In) -> Out + 'static,
    {
        Self(Rc::new(f))
    }

    #[inline]
    pub fn call(&self, input: In) -> Out {
        (self.0)(input)
    }
}

impl<In: 'static, Out: 'static> leptos::Callable<In, Out> for SafeCallback<In, Out> {
    #[inline]
    fn call(&self, input: In) -> Out {
        (self.0)(input)
    }
}

impl<In: 'static, Out: 'static, F> From<F> for SafeCallback<In, Out>
where
    F: Fn(In) -> Out + 'static,
{
    fn from(f: F) -> Self {
        SafeCallback::new(f)
    }
}

impl<In, Out> fmt::Debug for SafeCallback<In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SafeCallback").finish()
    }
}

impl<In, Out> PartialEq for SafeCallback<In, Out> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<In, Out> Eq for SafeCallback<In, Out> {}

pub type Callback<In, Out = ()> = SafeCallback<In, Out>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_safe_callback_execution_and_cloning() {
        let count = Rc::new(Cell::new(0));
        let count_clone = count.clone();

        let cb = Callback::new(move |n: i32| {
            count_clone.set(count_clone.get() + n);
        });

        let cb2 = cb.clone();
        cb.call(5);
        cb2.call(10);

        assert_eq!(count.get(), 15);
        assert_eq!(cb, cb2);
    }
}
