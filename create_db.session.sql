--select * from users;

--select * from roles


select r.* from roles r 
INNER JOIN users_roles ur on r.id = ur.roles_id
INNER JOIN users u on u.id = ur.users_id
WHERE u.username = 'admin_viewer'

select * from users_roles where 1=1 
and roles_id = (select * from roles where name ='' )
and users_id = (select * from users where username ='' )
 