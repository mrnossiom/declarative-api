use ast::types::*;
use lexer::rich::LiteralKind;
use session::{ident, sym, Ident, Span};
use thin_vec::thin_vec;

#[allow(clippy::too_many_lines)]
fn _paradigm_example_ast() -> Api {
	Api {
		attrs: thin_vec![
			Attribute {
				kind: AttrKind::DocComment(sym!(
					" This Api is a test for the documentation generator of the api"
				)),
				style: AttrStyle::Inner,
				id: AttrId::make_one(),
				span: Span::DUMMY,
			},
			Attribute {
				kind: AttrKind::DocComment(sym!(" This is a second line of comment")),
				style: AttrStyle::Inner,
				id: AttrId::make_one(),
				span: Span::DUMMY,
			},
			Attribute {
				kind: AttrKind::DocComment(sym!(" This is a third line of comment")),
				style: AttrStyle::Inner,
				id: AttrId::make_one(),
				span: Span::DUMMY,
			},
		],
		items: thin_vec![
			P(Item {
				attrs: thin_vec![],
				kind: ItemKind::Meta(Metadata {
					fields: thin_vec![
						P(PropertyDef {
							attrs: thin_vec![],
							ident: ident!("name", Span::DUMMY),
							expr: P(Expr {
								attrs: thin_vec![],
								kind: ExprKind::Literal(LiteralKind::Str, sym!("Wiro's API")),
								id: NodeId::DUMMY,
								span: Span::DUMMY
							}),
							id: NodeId::DUMMY,
							span: Span::DUMMY
						}),
						P(PropertyDef {
							attrs: thin_vec![],
							ident: ident!("description", Span::DUMMY),
							expr: P(Expr {
								attrs: thin_vec![],
								kind: ExprKind::Literal(
									LiteralKind::Str,
									sym!("This is the API of Wiro")
								),
								id: NodeId::DUMMY,
								span: Span::DUMMY
							}),
							id: NodeId::DUMMY,
							span: Span::DUMMY
						}),
						P(PropertyDef {
							attrs: thin_vec![],
							ident: ident!("version", Span::DUMMY),
							expr: P(Expr {
								attrs: thin_vec![],
								kind: ExprKind::Literal(LiteralKind::Str, sym!("1.0.0")),
								id: NodeId::DUMMY,
								span: Span::DUMMY
							}),
							id: NodeId::DUMMY,
							span: Span::DUMMY
						}),
						P(PropertyDef {
							attrs: thin_vec![],
							ident: ident!("name", Span::DUMMY),
							expr: P(Expr {
								attrs: thin_vec![],
								kind: ExprKind::Array(thin_vec![
									P(Expr {
										attrs: thin_vec![],
										kind: ExprKind::Literal(
											LiteralKind::Str,
											sym!("https://paradigm.lighton.ai/api/v1")
										),
										id: NodeId::DUMMY,
										span: Span::DUMMY
									}),
									P(Expr {
										attrs: thin_vec![],
										kind: ExprKind::Literal(
											LiteralKind::Str,
											sym!("https://paradigm-preprod.lighton.ai/api/v1")
										),
										id: NodeId::DUMMY,
										span: Span::DUMMY
									}),
									P(Expr {
										attrs: thin_vec![],
										kind: ExprKind::Literal(
											LiteralKind::Str,
											sym!("https://paradigm-dev.lighton.ai/api/v1")
										),
										id: NodeId::DUMMY,
										span: Span::DUMMY
									})
								]),
								id: NodeId::DUMMY,
								span: Span::DUMMY
							}),
							id: NodeId::DUMMY,
							span: Span::DUMMY
						}),
					],
				}),
				ident: ident!("", Span::DUMMY),
				id: NodeId::DUMMY,
				span: Span::DUMMY,
			}),
			P(Item {
				attrs: thin_vec![Attribute {
					kind: AttrKind::DocComment(sym!(" Imports the `builder.dapi` file")),
					style: AttrStyle::Outer,
					id: AttrId::make_one(),
					span: Span::DUMMY,
				}],
				ident: ident!("builder", Span::DUMMY),
				kind: ItemKind::Scope(ScopeKind::Unloaded),
				id: NodeId::DUMMY,
				span: Span::DUMMY,
			}),
			P(Item {
				attrs: thin_vec![],
				ident: ident!("dashboard", Span::DUMMY),
				kind: ItemKind::Scope(ScopeKind::Loaded {
					inline: true,
					items: thin_vec![P(Item {
						attrs: thin_vec![],
						ident: ident!("dashboard", Span::DUMMY),
						kind: ItemKind::Path(PathItem {
							kind: PathKind::Simple(ident!("dashboard", Span::DUMMY)),
							items: thin_vec![P(Item {
								attrs: thin_vec![],
								ident: ident!("", Span::DUMMY),
								kind: ItemKind::Headers(Headers {
									headers: thin_vec![
										P(FieldDef {
											attrs: thin_vec![
												Attribute {
													kind: AttrKind::DocComment(sym!(" # Safety")),
													style: AttrStyle::Outer,
													id: AttrId::make_one(),
													span: Span::DUMMY,
												},
												Attribute {
													kind: AttrKind::DocComment(sym!(
														" This is a comment"
													)),
													style: AttrStyle::Outer,
													id: AttrId::make_one(),
													span: Span::DUMMY,
												},
												Attribute {
													kind: AttrKind::DocComment(sym!(
														" This is a second line of comment"
													)),
													style: AttrStyle::Outer,
													id: AttrId::make_one(),
													span: Span::DUMMY,
												},
												// @description: "The API Key of the User"
												Attribute {
													kind: AttrKind::Meta(MetaAttr {
														ident: ident!("description", Span::DUMMY),
														expr: Some(P(Expr {
															attrs: thin_vec![],
															kind: ExprKind::Literal(
																LiteralKind::Str,
																sym!("The API Key of the User")
															),
															id: NodeId::DUMMY,
															span: Span::DUMMY,
														}))
													}),
													style: AttrStyle::Outer,
													id: AttrId::make_one(),
													span: Span::DUMMY,
												},
												// @prefix: "Api-Key"
												Attribute {
													kind: AttrKind::Meta(MetaAttr {
														ident: ident!("prefix", Span::DUMMY),
														expr: Some(P(Expr {
															attrs: thin_vec![],
															kind: ExprKind::Literal(
																LiteralKind::Str,
																sym!("Api-Key")
															),
															id: NodeId::DUMMY,
															span: Span::DUMMY,
														}))
													}),
													style: AttrStyle::Outer,
													id: AttrId::make_one(),
													span: Span::DUMMY,
												},
											],
											ident: ident!("Authorization", Span::DUMMY),
											ty: P(Ty {
												kind: TyKind::Path(Path {
													segments: thin_vec![PathSegment {
														ident: Ident {
															symbol: sym!("String"),
															span: Span::DUMMY,
														},
														id: NodeId::DUMMY,
													}],
													span: Span::DUMMY,
												}),
												id: NodeId::DUMMY,
												span: Span::DUMMY,
											}),
											id: NodeId::DUMMY,
											span: Span::DUMMY,
										}),
										P(FieldDef {
											// @description: "The Model of the User"
											attrs: thin_vec![Attribute {
												kind: AttrKind::Meta(MetaAttr {
													ident: ident!("description", Span::DUMMY),
													expr: Some(P(Expr {
														attrs: thin_vec![],
														kind: ExprKind::Literal(
															LiteralKind::Str,
															sym!("The Model of the User")
														),
														id: NodeId::DUMMY,
														span: Span::DUMMY,
													}))
												}),
												style: AttrStyle::Outer,
												id: AttrId::make_one(),
												span: Span::DUMMY,
											}],
											ident: ident!("X-Model", Span::DUMMY),
											ty: P(Ty {
												kind: TyKind::Path(Path {
													segments: thin_vec![PathSegment {
														ident: ident!("String", Span::DUMMY),
														id: NodeId::DUMMY,
													}],
													span: Span::DUMMY,
												}),
												id: NodeId::DUMMY,
												span: Span::DUMMY,
											}),
											id: NodeId::DUMMY,
											span: Span::DUMMY,
										}),
									],
								}),
								id: NodeId::DUMMY,
								span: Span::DUMMY,
							})],
						}),
						id: NodeId::DUMMY,
						span: Span::DUMMY,
					})],
					span: Span::DUMMY,
				}),
				id: NodeId::DUMMY,
				span: Span::DUMMY,
			}),
		],
		id: NodeId::ROOT,
		span: Span::DUMMY,
	}
}
