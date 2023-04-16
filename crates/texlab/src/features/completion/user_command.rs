use base_db::DocumentData;

use crate::util::cursor::CursorContext;

use super::builder::CompletionBuilder;

pub fn complete<'db>(
    context: &'db CursorContext,
    builder: &mut CompletionBuilder<'db>,
) -> Option<()> {
    let range = context.cursor.command_range(context.offset)?;

    for document in &context.project.documents {
        let DocumentData::Tex(data) = &document.data else { continue };
        for (_, name) in data.semantics.commands.iter().filter(|(r, _)| *r != range) {
            builder.user_command(range, name);
        }
    }

    Some(())
}